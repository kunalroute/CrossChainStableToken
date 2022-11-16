// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface PriceSource {
    function latestAnswer() external view returns (uint256);
}
