// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "./library/CrosschainERC20.sol";

contract StableCoin is XERC20 {
    constructor(
        string memory _tokenName,
        string memory _symbol,
        address _owner
    ) XERC20(_tokenName, _symbol, _owner) {}

    function mint(address _account, uint256 _amount) external onlyTreasury {
        _mint(_account, _amount);
    }

    function burn(address _account, uint256 _amount) external onlyTreasury {
        _burn(_account, _amount);
    }
}
