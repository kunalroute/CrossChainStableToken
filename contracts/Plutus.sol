// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "./PriceSource.sol";

import "./PawnVault.sol";
import "./Treasury.sol";

import "evm-gateway-contract/contracts/IGateway.sol";
import "evm-gateway-contract/contracts/IApplication.sol";

/**
    @title Cross-chain Collateral provider
    @author Router Protocol
    @notice This contract will handle Cross-chain Collateral
*/
contract Plutus is IApplication, ReentrancyGuard, PawnVault, Ownable {
    using Counters for Counters.Counter;
    using SafeMath for uint256;
    using SafeERC20 for ERC20;

    IGateway public gatewayContract;

    Counters.Counter private _tokenIdCounter;

    uint256 public _minCollateralPercent;
    uint256 public closingFee;
    uint256 public minDebt;
    uint256 public pawnId;

    mapping(uint256 => uint256) public vaultCollateral;
    mapping(uint256 => uint256) public vaultDebt;

    uint256 public gainRatio;
    uint256 public tokenPeg;
    address public treasury;
    address public collateral;
    address public stableCoin;

    // uint256 public priceSourceDecimals;
    uint256 public totalBorrowed;

    address public tokenPriceSource;
    mapping(address => uint256) public unClaimedLiquidation;
    string public routerBridgeContract;

    event CreateVault(uint256 vaultID, address creator);
    event DestroyVault(uint256 vaultID);
    event DepositCollateral(uint256 vaultID, uint256 amount);
    event WithdrawCollateral(uint256 vaultID, uint256 amount);
    event BorrowToken(uint256 vaultID, uint256 amount);

    event XBorrowToken(uint256 vaultID, uint256 amount);

    event XInvestAndBorrow(
        uint8 chainID,
        uint256 vaultID,
        uint256 amount,
        address receiver
    );
    event InvestAndBorrow(uint256 vaultID, uint256 amount);
    event PayBackToken(uint256 vaultID, uint256 amount);
    event XPayBackToken(uint256 vaultID, uint256 amount);
    event LiquidateVault(
        uint256 vaultID,
        address owner,
        address buyer,
        uint256 debtRepaid,
        uint256 collateralLiquidated,
        uint256 closingFee
    );


    constructor(
        uint256 minCollateralPercent,
        string memory name,
        string memory symbol,
        string memory baseURI,
        address _stableCoin,
        address _collateral,
        address _treasury,
        address _tokenPriceSource,
        address payable gatewayAddress,
        string memory _routerBridgeContract
    ) PawnVault(name, symbol, baseURI) {
        assert(minCollateralPercent != 0);

        closingFee = 50; // 0.5% * 1000
        gainRatio = 1100; // /10 so 1.1

        _minCollateralPercent = minCollateralPercent;
        tokenPeg = 10**8;
        collateral = _collateral;
        stableCoin = _stableCoin;
        treasury = _treasury;
        tokenPriceSource = _tokenPriceSource;
        pawnId = createVault();
        gatewayContract = IGateway(gatewayAddress);
        routerBridgeContract = _routerBridgeContract;
    }

    modifier onlyVaultOwner(uint256 vaultID) {
        require(_exists(vaultID), "Vault does not exist");
        require(ownerOf(vaultID) == msg.sender, "Vault is not owned by you");
        _;
    }

    modifier isSelf() {
        require(msg.sender == address(this), "only this contract");
        _;
    }

    /**
        @notice Sets the price source address for the token.
        @param _source the new one price source token
     */
    function setTokenPriceSource(address _source) external onlyOwner {
        tokenPriceSource = _source;
    }

    /**
        @notice Sets the minimum Collateral Percent.
        @param _percent the minimum Collateral Percent.
     */
    function setMinCollateralPercent(uint256 _percent) external onlyOwner {
        _minCollateralPercent = _percent;
    }

    /**
        @notice check existance of the vault
        @param vaultID vault id
        @return bool true if the vault exists.
     */
    function exists(uint256 vaultID) external view returns (bool) {
        return _exists(vaultID);
    }

    /**
        @notice fetches the closing fee price.
     */
    function getClosingFee() external view returns (uint256) {
        return closingFee;
    }

    /**
        @notice fetches the stable token price.
     */
    function getStableTokenPrice() public view returns (uint256) {
        return tokenPeg;
    }

    /**
        @notice get the token price from source oracle.
     */
    function getTokenPrice() public view returns (uint256 price) {
        price = PriceSource(tokenPriceSource).latestAnswer();
    }

    /**
        @notice check status of the vault
        @param _vaultId vault id
        @return bool Returns false if the vault is risky else true.
     */
    function vaultStatus(uint256 _vaultId) public view returns (bool) {
        uint256 debt = vaultDebt[_vaultId];
        uint256 collateralAmount = vaultCollateral[_vaultId];
        (
            uint256 collateralValueTimes100,
            uint256 debtValue
        ) = calculateCollateralProperties(collateralAmount, debt);

        if (debtValue == 0) {
            return true;
        }

        uint256 collateralPercentage = collateralValueTimes100.div(debtValue);

        return collateralPercentage >= _minCollateralPercent;
    }

    function calculateCollateralProperties(uint256 _collateral, uint256 _debt)
        public
        view
        returns (uint256, uint256)
    {
        uint256 collateralValue = _collateral.mul(getTokenPrice()).mul(
            10 **
                (
                    uint256(ERC20(stableCoin).decimals()).sub(
                        uint256(ERC20(collateral).decimals())
                    )
                )
        );

        uint256 debtValue = _debt.mul(getStableTokenPrice());

        uint256 collateralValueTimes100 = collateralValue.mul(100);

        return (collateralValueTimes100, debtValue);
    }

    function isValidCollateral(uint256 _collateral, uint256 debt)
        private
        view
        returns (bool)
    {
        (
            uint256 collateralValueTimes100,
            uint256 debtValue
        ) = calculateCollateralProperties(_collateral, debt);
        if (debt == 0) {
            return true;
        }
        uint256 collateralPercentage = collateralValueTimes100.div(debtValue);

        return collateralPercentage >= _minCollateralPercent;
    }

    /**
        @notice check status for the borowwed monay
        @param _vaultId vault id
        @return bool Returns false if the vault becomses risky after the new borrowing else true.
     */
    function borrowStatus(uint256 _vaultId, uint256 _amount)
        public
        view
        returns (bool)
    {
        uint256 debt = vaultDebt[_vaultId];
        debt += _amount;
        uint256 debtValue = debt.mul(getStableTokenPrice());
        uint256 collateralAmount = vaultCollateral[_vaultId];
        uint256 collateralValue = collateralAmount.mul(getTokenPrice());
        uint256 collateralValueTimes100 = collateralValue.mul(100);
        uint256 collateralPercentage = collateralValueTimes100.div(debtValue);

        return collateralPercentage >= _minCollateralPercent;
    }

    /**
        @notice create a new vault 
        @return uint256 Returns the newly created vault ID in response.
     */
    function createVault() public returns (uint256) {
        uint256 id = _tokenIdCounter.current();
        _mint(msg.sender, id);
        _tokenIdCounter.increment();
        emit CreateVault(id, msg.sender);
        return id;
    }

    /**
        @notice destroy a existing vault 
        @param vaultID a existing vault ID.
     */
    function destroyVault(uint256 vaultID)
        external
        onlyVaultOwner(vaultID)
        nonReentrant
    {
        require(vaultDebt[vaultID] == 0, "Vault has outstanding debt");

        if (vaultCollateral[vaultID] != 0) {
            // withdraw leftover collateral
            Treasury(treasury).transferFromVault(
                ownerOf(vaultID),
                vaultCollateral[vaultID],
                collateral
            );
        }

        _burn(vaultID);

        delete vaultCollateral[vaultID];
        delete vaultDebt[vaultID];

        emit DestroyVault(vaultID);
    }

    /**
        @notice This function will accept the collateral coin as deposit. 
        @param vaultID the vault ID.
        @param amount the deposited amount
     */
    function depositCollateral(uint256 vaultID, uint256 amount)
        public
        onlyVaultOwner(vaultID)
        nonReentrant
    {
        ERC20(collateral).transferFrom(msg.sender, treasury, amount);
        uint256 newCollateral = vaultCollateral[vaultID].add(amount);
        assert(newCollateral >= vaultCollateral[vaultID]);

        vaultCollateral[vaultID] = newCollateral;

        emit DepositCollateral(vaultID, amount);
    }

    /**
        @notice This function will withdraw the collateral coin from the user's account. 
        @param vaultID the vault ID.
        @param amount the deposited amount
     */
    function withdrawCollateral(uint256 vaultID, uint256 amount)
        external
        onlyVaultOwner(vaultID)
        nonReentrant
    {
        require(
            vaultCollateral[vaultID] >= amount,
            "Vault does not have enough collateral"
        );

        uint256 newCollateral = vaultCollateral[vaultID].sub(amount);

        if (vaultDebt[vaultID] != 0) {
            require(
                isValidCollateral(newCollateral, vaultDebt[vaultID]),
                "Withdrawal would put vault below minimum collateral percentage"
            );
        }

        vaultCollateral[vaultID] = newCollateral;
        Treasury(treasury).transferFromVault(msg.sender, amount, collateral);
        emit WithdrawCollateral(vaultID, amount);
    }

    /**
        @notice This function will create a vault and deposit the amount and then borrow some amount . 
        @param _borrowamount the borrow amount.
        @param _depositAmount the deposited amount
     */
    function investAndBorrow(uint256 _borrowamount, uint256 _depositAmount)
        external
    {
        uint256 _id = createVault();
        depositCollateral(_id, _depositAmount);
        borrowToken(_borrowamount, _id);
        emit InvestAndBorrow(_id, _borrowamount);
    }

    /**
        @notice This function will create a vault and deposit the amount and then borrow some amount in dest chain. 
        @param _borrowamount the borrow amount.
        @param _depositAmount the deposited amount
        @param _chainId the chain id where we want the borrowed money
        @param _receiver the borrow monay receiver.
        @param gasLimit the gasLimit for the function execution.
        @param gasPrice the gasPrice for the function execution.
     */
    function xInvestAndBorrow(
        uint256 _borrowamount,
        uint256 _depositAmount,
        uint8 _chainId,
        address _receiver,
        uint256 gasLimit,
        uint256 gasPrice
    ) external {
        uint256 _id = createVault();
        depositCollateral(_id, _depositAmount);
        xBorrowToken(
            _chainId,
            _borrowamount,
            _id,
            _receiver,
            gasLimit,
            gasPrice
        );
        emit XInvestAndBorrow(_chainId, _id, _borrowamount, _receiver);
    }

    function _borrowToken(uint256 _amount, uint256 _vaultID) internal {
        require(_amount > 0, "Must borrow non-zero amount");

        uint256 newDebt = vaultDebt[_vaultID].add(_amount);

        require(
            isValidCollateral(vaultCollateral[_vaultID], newDebt),
            "Borrow would put vault below minimum collateral percentage"
        );

        require(
            (vaultDebt[_vaultID]).add(_amount) >= minDebt,
            "Vault debt can't be under minDebt"
        );

        vaultDebt[_vaultID] = newDebt;

        // stableCoin
        totalBorrowed = totalBorrowed.add(_amount);
        // Treasury(treasury).mintStableCoin(_amount,msg.sender,stableCoin);
        emit BorrowToken(_vaultID, _amount);
    }

    /**
        @notice This function will  borrow some amount . 
        @param _vaultID the vault ID.
        @param _amount the amount
     */
    function borrowToken(uint256 _amount, uint256 _vaultID)
        public
        nonReentrant
        onlyVaultOwner(_vaultID)
    {
        _borrowToken(_amount, _vaultID);
        Treasury(treasury).mintStableCoin(_amount, msg.sender, stableCoin);
        emit BorrowToken(_vaultID, _amount);
    }

    //outgoing
    /**
        @notice This function will  borrow some amount on the dest chain side. 
        @param _vaultID the vault ID.
        @param _amount the amount
        @param _chainID the dest chain id
        @param _receiver the receiver address
        @param gasLimit the max required gas supply.
        @param gasPrice the gasPrice for the function execution.
     */
    function xBorrowToken(
        uint8 _chainID,
        uint256 _amount,
        uint256 _vaultID,
        address _receiver,
        uint256 gasLimit,
        uint256 gasPrice    
    ) public nonReentrant onlyVaultOwner(_vaultID) { //returns (bool, bytes32) {
        _borrowToken(_amount, _vaultID);


        /*bytes memory data = abi.encode(_amount, _receiver);
        bytes4 _interface = bytes4(keccak256("xMint(uint256,address)"));
        (bool success, bytes32 hash) = routerSend(
            _chainID,
            _interface,
            data,
            gasLimit,
            gasPrice
        );*/


        bytes memory data = abi.encode(_chainID, _amount, _receiver);
        bytes memory payload = abi.encode(0, data);  // 0 -> xMint(uint256,address)

        gatewayContract.requestToRouter(payload, routerBridgeContract);


        emit XBorrowToken(_vaultID, _amount);
        //return (success, hash);
    }

    //Incoming function before that we have already burn stable

    function _updatePayBackToken(uint256 vaultID, uint256 amount) internal {
        uint256 _closingFee = amount
            .mul(closingFee)
            .mul(getStableTokenPrice())
            .div(getTokenPrice().mul(10000));

        //stableCoin
        vaultDebt[vaultID] = vaultDebt[vaultID].sub(amount);
        vaultCollateral[vaultID] = vaultCollateral[vaultID].sub(_closingFee);
        vaultCollateral[pawnId] = vaultCollateral[pawnId].add(_closingFee);
        totalBorrowed = totalBorrowed.sub(amount);
    }

    /**
        @notice This function will  payBack the borrowed stable coins . 
        @param vaultID the vault ID.
        @param amount the amount
     */
    function payBackToken(uint256 vaultID, uint256 amount) public {
        require(
            ERC20(stableCoin).balanceOf(msg.sender) >= amount,
            "Token balance too low"
        );

        require(
            vaultDebt[vaultID] >= amount,
            "Vault debt less than amount to pay back"
        );
        _updatePayBackToken(vaultID, amount);
        Treasury(treasury).burnStableCoin(amount, msg.sender, stableCoin);
        emit PayBackToken(vaultID, amount);
    }

    /**
        @notice This function will  payBack the borrowed stable coins on the source side. 
        @param vaultID the vault ID.
        @param amount the amount
     */
    function xpayBackToken(uint256 vaultID, uint256 amount) external isSelf {
        //Check if amount value is equal or greater than collateral value based of this mint or payback
        if (amount > vaultDebt[vaultID]) {
            uint256 delta = amount - vaultDebt[vaultID];
            amount = vaultDebt[vaultID];
            //mint delta
            Treasury(treasury).mintStableCoin(delta, msg.sender, stableCoin);
        }

        _updatePayBackToken(vaultID, amount);
        // Treasury(treasury).burnStableCoin(amount,msg.sender,stableCoin);
        emit PayBackToken(vaultID, amount);
    }

    //outgoinf function 2
    /**
        @notice This function will  payBack the borrowed stable coins form the destination side. 
        @param _chainID the source chain ID.
        @param _vaultID the vault ID
        @param gasLimit the max gas supplu limit.
        @param _amount the amount
     */
    function xPayback(
        uint8 _chainID,
        uint256 _vaultID,
        uint256 _amount,
        uint256 gasLimit,
        uint256 gasPrice
    ) external nonReentrant { //returns (bool, bytes32) {
        require(
            ERC20(stableCoin).balanceOf(msg.sender) >= _amount,
            "Token balance too low"
        );

        Treasury(treasury).burnStableCoin(_amount, msg.sender, stableCoin);

        /*bytes memory data = abi.encode(_amount, _vaultID);
        bytes4 _interface = bytes4(keccak256("xpayBackToken(uint256,uint256)"));
        (bool success, bytes32 hash) = routerSend(
            _chainID,
            _interface,
            data,
            gasLimit,
            gasPrice
        );*/

        bytes memory data = abi.encode(_chainID, _amount, _vaultID);
        bytes memory payload = abi.encode(1, data);  // 1 -> xpayBackToken(uint256,uint256)

        gatewayContract.requestToRouter(payload, routerBridgeContract);

        emit XPayBackToken(_vaultID, _amount);
        //return (success, hash);
    }

    // // Hash that is returned while calling routerSend function
    // function replayTx(
    //     bytes32 hash,
    //     uint256 gasLimit,
    //     uint256 gasPrice
    // ) external onlyOwner {
    //     routerReplay(hash, gasLimit, gasPrice);
    // }

    /**
        @notice This function will  mint the stable coins for the to address. 
        @param _to the receiver address
        @param _amount the amount
     */
    function xMint(uint256 _amount, address _to) external isSelf {
        Treasury(treasury).mintStableCoin(_amount, _to, stableCoin);
    }

    // function xBurn(uint256 _amount, address _to) external isSelf {
    //     Treasury(treasury).burnStableCoin(_amount, _to, stableCoin);
    // }

    //incoming function
    /*function _routerSyncHandler(bytes4 _interface, bytes memory _data)
        internal
        virtual
        override
        returns (bool, bytes memory)
    {
        bytes4 _mintInterface = bytes4(keccak256("xMint(uint256,address)"));
        bytes4 _xpayBackTokenInterface = bytes4(
            keccak256("xpayBackToken(uint256,uint256)")
        );

        if (_interface == _mintInterface) {
            (uint256 _amount, address _to) = abi.decode(
                _data,
                (uint256, address)
            );
            (bool success, bytes memory returnData) = address(this).call(
                abi.encodeWithSelector(_interface, _amount, _to)
            );
            return (success, returnData);
        }
        if (_interface == _xpayBackTokenInterface) {
            (uint256 _amount, uint256 _vaultId) = abi.decode(
                _data,
                (uint256, uint256)
            );
            (bool success, bytes memory returnData) = address(this).call(
                abi.encodeWithSelector(_interface, _vaultId, _amount)
            );
            return (success, returnData);
        }
    }*/

    function handleRequestFromRouter(string memory sender, bytes memory payload) override external {
        // This check is to ensure that the contract is called from the Gateway only.
        require(msg.sender == address(gatewayContract));

        // bytes4 _mintInterface = bytes4(keccak256("xMint(uint256,address)"));
        // bytes4 _xpayBackTokenInterface = bytes4(
        //     keccak256("xpayBackToken(uint256,uint256)")
        // );

        // methodType = method to call, data = method params
        (uint8 _methodType, bytes memory _data) = abi.decode(payload, (uint8, bytes));

        // mint
        if (_methodType == 0) {
            (uint256 _amount, address _to) = abi.decode(_data, (uint256, address));

            this.xMint(_amount, _to);
            // (bool success, bytes memory returnData) = address(this).call(abi.encodeWithSelector(_mintInterface, _amount, _to));
        } else if (_methodType == 1) {
            (uint256 _amount, uint256 _vaultId) = abi.decode(_data, (uint256, uint256));
            
            this.xpayBackToken(_vaultId, _amount);
            // (bool success, bytes memory returnData) = address(this).call(abi.encodeWithSelector(_xpayBackTokenInterface, _vaultId, _amount));
        }
        //require(keccak256(abi.encodePacked(sampleStr)) != keccak256(abi.encodePacked("")));
        //greeting = sampleStr;
        //emit RequestFromRouterEvent(sender, payload);
    }

    /**
        @notice This function will  transfer the unclaimed Liquidation money to the user. 
     */
    function getPaid() public nonReentrant {
        require(
            unClaimedLiquidation[msg.sender] != 0,
            "Don't have anything for you."
        );
        uint256 amount = unClaimedLiquidation[msg.sender];
        unClaimedLiquidation[msg.sender] = 0;
        Treasury(treasury).transferFromVault(msg.sender, amount, collateral);
    }

    /**
        @notice This function will check the liquidation value to make the vault safe again. 
        @param vaultID the vault ID.
     */
    function calculateLiquidationValue(uint256 vaultID)
        public
        view
        returns (uint256)
    {
        if (vaultStatus(vaultID)) {
            return 0;
        }
        uint256 debt = vaultDebt[vaultID];
        uint256 collateralAmount = vaultCollateral[vaultID];

        // This function will return doller eqvivalent values of collateral and dept
        (
            uint256 collateralValueTimes100,
            uint256 debtValue
        ) = calculateCollateralProperties(collateralAmount, debt);
        if (debtValue == 0) {
            return 0;
        }
        uint256 liquidationRatio = _minCollateralPercent +
            _minCollateralPercent.div(10);
        uint256 rhsValue = debtValue.mul(liquidationRatio).sub(
            collateralValueTimes100
        );
        uint256 lhsValue = liquidationRatio.sub(gainRatio.div(10));
        uint256 xValue = rhsValue.div(lhsValue);

        if ((debtValue - xValue) <= minDebt) {
            xValue = debtValue;
        }

        return xValue.div(getStableTokenPrice());
    }

    /**
        @notice This function will check the liquidation cost of a vault. 
        @param vaultID the vault ID.
     */
    function checkCost(uint256 vaultID) public view returns (uint256) {
        if (
            vaultCollateral[vaultID] == 0 ||
            vaultDebt[vaultID] == 0 ||
            !checkLiquidation(vaultID)
        ) {
            return 0;
        }

        uint256 requiredDebt = calculateLiquidationValue(vaultID);
        return requiredDebt;
    }

    /**
        @notice This function will check the liquidation benefit if the vault is ready for liquidation. 
        @param vaultID the vault ID.
     */
    function checkExtract(uint256 vaultID) public view returns (uint256) {
        if (vaultCollateral[vaultID] == 0 || !checkLiquidation(vaultID)) {
            return 0;
        }

        uint256 requiredDebt = calculateLiquidationValue(vaultID);
        requiredDebt = requiredDebt.mul(getStableTokenPrice());
        if (requiredDebt == 0) {
            return 0;
        }

        return requiredDebt.mul(gainRatio).div(1000).div(getTokenPrice());
    }

    /**
        @notice This function will calculate the vault collateral percentage. 
        @param vaultID the vault ID.
     */
    function checkCollateralPercentage(uint256 vaultID)
        public
        view
        returns (uint256)
    {
        require(_exists(vaultID), "Vault does not exist");

        if (vaultCollateral[vaultID] == 0 || vaultDebt[vaultID] == 0) {
            return 0;
        }
        (
            uint256 collateralValueTimes100,
            uint256 debtValue
        ) = calculateCollateralProperties(
                vaultCollateral[vaultID],
                vaultDebt[vaultID]
            );

        return collateralValueTimes100.div(debtValue);
    }

    /**
        @notice This function will check the vault can be liquidated.
        @param vaultID the user's vaultID
        @return bool True if the vault can be liquidated else False
     */
    function checkLiquidation(uint256 vaultID) public view returns (bool) {
        require(_exists(vaultID), "Vault does not exist");

        if (vaultCollateral[vaultID] == 0 || vaultDebt[vaultID] == 0) {
            return false;
        }

        (
            uint256 collateralValueTimes100,
            uint256 debtValue
        ) = calculateCollateralProperties(
                vaultCollateral[vaultID],
                vaultDebt[vaultID]
            );

        uint256 collateralPercentage = collateralValueTimes100.div(debtValue);

        if (collateralPercentage < _minCollateralPercent) {
            return true;
        } else {
            return false;
        }
    }

    /**
        @notice This function will liquidate the risky vault.
        @param vaultID the user's vaultID
     */
    function liquidateVault(uint256 vaultID) external {
        require(_exists(vaultID), "Vault does not exist");

        (
            uint256 collateralValueTimes100,
            uint256 debtValue
        ) = calculateCollateralProperties(
                vaultCollateral[vaultID],
                vaultDebt[vaultID]
            );

        uint256 collateralPercentage = collateralValueTimes100.div(debtValue);

        require(
            collateralPercentage < _minCollateralPercent,
            "Vault is not below minimum collateral percentage"
        );

        // debtValue = debtValue.div(10**priceSourceDecimals);

        uint256 requiredDebt = calculateLiquidationValue(vaultID);

        require(
            ERC20(stableCoin).balanceOf(msg.sender) >= requiredDebt,
            "Token balance too low to pay off outstanding debt"
        );

        //stableCoin
        ERC20(stableCoin).transferFrom(msg.sender, treasury, requiredDebt);
        totalBorrowed = totalBorrowed.sub(requiredDebt);

        uint256 extract = checkExtract(vaultID);

        vaultDebt[vaultID] = vaultDebt[vaultID].sub(requiredDebt); // we paid back half of its debt.

        uint256 _closingFee = requiredDebt
            .mul(closingFee)
            .mul(getStableTokenPrice())
            .div(getTokenPrice().mul(10000));

        vaultCollateral[vaultID] = vaultCollateral[vaultID].sub(_closingFee);
        vaultCollateral[pawnId] = vaultCollateral[pawnId].add(_closingFee);

        // deduct the amount from the vault's collateral
        vaultCollateral[vaultID] = vaultCollateral[vaultID].sub(extract);

        // let liquidator take the collateral
        unClaimedLiquidation[msg.sender] = unClaimedLiquidation[msg.sender].add(
            extract
        );

        emit LiquidateVault(
            vaultID,
            ownerOf(vaultID),
            msg.sender,
            requiredDebt,
            extract,
            _closingFee
        );
    }

    //ignore
    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC721)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
