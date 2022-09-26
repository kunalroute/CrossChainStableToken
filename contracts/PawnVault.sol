// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract PawnVault is ERC721 {
    string public uri;

    constructor(
        string memory name,
        string memory symbol,
        string memory _uri
    ) ERC721(name, symbol) {
        uri = _uri;
    }

    function getTokenURI(uint256 tokenId) public view returns (string memory) {
        require(_exists(tokenId));

        return uri;
    }
}
