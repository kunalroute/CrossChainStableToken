// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IPawnMaster {
    function setTokenPriceSource(address _token, address _source) external;
    function getTokenURI(uint256 tokenId) external view returns (string memory);
    function getDebtCeiling() external view returns (uint256);
    function exists(uint256 vaultID) external view returns (bool);
    function getClosingFee() external view returns (uint256);
    function getStableTokenPrice() external view returns (uint256);
    function getEthPriceSource() external view returns (uint256);
    function getTokenPrice(address _token) external view returns (uint256 _price);
    function totalCollateralValue(uint256 _vaultId) external view returns (uint256, uint256);
    function vaultStatus(uint256 _vaultId) external view returns (bool);
    function collateralWithdrawlStatus(
        uint256 _vaultId,
        address _token,
        uint256 _amount
    ) external view returns (bool);
    function borrowStatus(uint256 _vaultId, uint256 _amount)
        external
        view
        returns (bool);
    function createVault() external returns (uint256);
    function destroyVault(uint256 vaultID) external;
    function depositCollateral(
        uint256 vaultID,
        uint256 amount,
        address _token
    ) external;
    function withdrawCollateral(
        uint256 vaultID,
        uint256 amount,
        address _token
    ) external;
    function borrowToken(uint256 vaultID, uint256 amount)
        external;
    function payBackToken(uint256 vaultID, uint256 amount) external;
    function getPaid(address _token) external;
    function checkCost(uint256 vaultID) external view returns (uint256);
    function checkExtract(uint256 vaultID,address _token) external view returns (uint256);
    function checkExtract1(uint256 vaultID,address _token,uint _totaltokens) external view returns (uint256);
    function checkCollateralPercentage(uint256 vaultID)
        external
        view
        returns (uint256);
    function liquidateVault(uint256 vaultID) external;
    function getActiveTokens(address[]memory _tokens,uint vaultID) external view returns(uint256 tokenCount);

}