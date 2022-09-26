// SPDX-License-Identifier: MIT
pragma solidity ^0.8.2;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/draft-EIP712.sol";

/**
    @title Handles generic deposits and deposit executions.
    @author ChainSafe Systems.
    @notice This contract is intended to be used with the Bridge contract.
 */
contract GenericHandler is EIP712 {
    using ECDSA for bytes32;

    constructor() EIP712("RouterSync", "0.0.1") {}

    // Map Address Functions
    struct RouterLinker {
        address _rSyncContract;
        uint8 _chainID;
        address _linkedContract;
        uint8 linkerType;
        uint256 timelimit;
    }

    function MapContract(RouterLinker calldata linker, bytes memory _sign)
        external
    {}

    function UnMapContract(RouterLinker calldata linker, bytes memory _sign)
        external
    {}

    function _hash(RouterLinker calldata linker)
        internal
        view
        returns (bytes32)
    {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        keccak256(
                            "RouterLinker(address _rSyncContract,uint8 _chainID,address _linkedContract,uint8 linkerType)"
                        ),
                        linker._rSyncContract,
                        linker._chainID,
                        linker._linkedContract,
                        linker.linkerType
                    )
                )
            );
    }

    function GenHash(RouterLinker calldata linker)
        external
        view
        returns (bytes32)
    {
        return _hash(linker);
    }

    function _verify(RouterLinker calldata linker, bytes memory signature)
        internal
        view
        returns (address)
    {
        bytes32 digest = _hash(linker);
        return digest.toEthSignedMessageHash().recover(signature);
    }

    function verifySignature(
        RouterLinker calldata voucher,
        bytes memory signature
    ) external view returns (address) {
        return _verify(voucher, signature);
    }
    // Map Address Functions
}
