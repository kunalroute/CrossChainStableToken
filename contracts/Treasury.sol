// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "./StableCoin.sol";

contract Treasury is Ownable, ReentrancyGuard, Pausable {
    using SafeERC20 for IERC20;

    mapping(address => bool) public whitelsited;

    event TreasuryPaused();

    event TreasuryUnPaused();

    constructor() Pausable() {}

    function authorize(address _owner) external onlyOwner {
        whitelsited[_owner] = true;
    }

    function pauseTreasury() external onlyOwner {
        _pause();

        emit TreasuryPaused();
    }

    function unPauseTreasury() external onlyOwner {
        _unpause();

        emit TreasuryUnPaused();
    }

    function transferFromVault(
        address _to,
        uint256 _amount,
        address _token
    ) external whenNotPaused onlyPlutus {
        IERC20(_token).safeTransfer(_to, _amount);
    }

    function mintStableCoin(
        uint256 _amount,
        address _to,
        address _token
    ) external whenNotPaused onlyPlutus {
        StableCoin(_token).mint(_to, _amount);
    }

    function burnStableCoin(
        uint256 _amount,
        address _from,
        address _token
    ) external whenNotPaused onlyPlutus {
        StableCoin(_token).burn(_from, _amount);
    }

    function approveFeesOnStableCoin(
        address token,
        address feeToken,
        uint256 amount
    ) external onlyPlutus {
        StableCoin(token)._approveFees(feeToken, amount);
    }

    modifier onlyPlutus() {
        require(whitelsited[msg.sender], "Caller is not Plutus");
        _;
    }
}
