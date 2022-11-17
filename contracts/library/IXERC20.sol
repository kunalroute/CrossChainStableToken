// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IXERC20 is IERC20 {
    /**
     * @dev Moves `amount` tokens from the caller's account to `to` in `chainId`.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a  event.
     */
    function xTransfer(
        uint256 chainId,
        address to,
        uint256 amount,
        address destContractAddress
    ) external;

    event XTransfer(uint256 chainId, address to, uint256 amount);
    event XReceive(address to, uint256 amount);
}
