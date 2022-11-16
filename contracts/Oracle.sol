// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;
import "./PriceSource.sol";

contract Oracle is PriceSource {
    function latestAnswer() external pure returns (uint256) {
        return 1300;
    }
}
