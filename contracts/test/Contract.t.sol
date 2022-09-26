// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.13;

// import "@openzeppelin/contracts/utils/Strings.sol";

// import "ds-test/test.sol";
// import "../Plutus.sol";
// import "../IPriceSourceAll.sol";

// contract PlutusTest is DSTest {
//     Plutus private plutus;

//     IPriceSourceAll private iPriceSourceAll;
    
//     function setUp() public {
//         // iPriceSourceAll = IPriceSourceAll(address(0xB0409AEF2E95843Ee7faD38637e5E3Af4289A5A1));
//         plutus = new Plutus(
//             200,
//             "SetupName",
//             "SUP",
//             address(0x844C595E9cDf9DE66D76cc326D78035297A2d8f9),
//             address(0x12ac7A4ea72352c143e3e21C2FD64Ae798b69131),
//             "baseURI",
//             address(0xD5e0bcD5147E070CeC09473A78D47208994d0C33),
//             address(0x339c21Aa02B9247a2f84aC153709a46788482A14)
//         );
//     }

//     function testExample() public {
//         assertTrue(true);
//         uint a=2;
//         assertEq(2,a);
//         emit log_uint(a);
//     }

//     function testCreateVault() public {
//         // create vault
//         uint256 vaultId = plutus.createVault();
//         assertEq(1, vaultId);

//         // create vault
//         vaultId = plutus.createVault();
//         assertEq(2, vaultId);
//     }

//     function testDestroyVault() public {
//         // create vault
//         uint256 vaultId = plutus.createVault();
//         assertEq(1, vaultId);

//         // create vault
//         vaultId = plutus.createVault();
//         assertEq(2, vaultId);


//     }

//     function testRawVaultStatus() public {
//         // create vault
//         uint256 vaultId = plutus.createVault();
//         assertEq(1, vaultId);

//         // create vault
//         vaultId = plutus.createVault();
//         assertEq(2, vaultId);

        
//         // check empty vault status
//         bool vaultStatus = plutus.vaultStatus(1);
//         assertTrue(vaultStatus);

//         // check destroyed vault status
//         vaultStatus = plutus.vaultStatus(2);
//         assertTrue(vaultStatus);

//         // check non-existing vault status
//         vaultStatus = plutus.vaultStatus(3);
//         assertTrue(vaultStatus);
//     }

//     function testRawVaultOperations() public {
//         // create vault
//         uint256 vaultId = plutus.createVault();
//         assertEq(1, vaultId);

//         // check Token Price
//         // uint256 tokenPrice = iPriceSourceAll.latestAnswer();
//         // vm.mockcall(
//         //     address(0x339c21Aa02B9247a2f84aC153709a46788482A14), 
//         //     abi.encodeWithSelector(IPriceSourceAll.latestAnswer.selector), 
//         //     abi.encode(2029384778)
//         // );
//         // uint256 tokenPrice = plutus.getTokenPrice();
//         // assertGt(tokenPrice, 0);
        
//         // check empty vault status
//         // bool vaultStatus = plutus.borrowStatus(vaultId, 100);
//         // assertTrue(!vaultStatus);

//         // destroy vault
//         // plutus.destroyVault(vaultId);
        

//         // check destroyed vault deposit
//         // plutus.depositCollateral(vaultId, 100);
//         // assertTrue(!vaultStatus);
//     }

//     function testLiquidationMath() public {
//         // create vault
//         uint256 vaultId = plutus.createVault();
//         assertEq(1, vaultId);

//         // pre collateral value
//         (uint256 preValue, ) = plutus.totalCollateralValue(vaultId);
//         emit log_uint(preValue);
//         assert(preValue == 18000);

//         // assert(postValue == 100);
//         uint256 xValue = plutus.calculateLiquidationValue(vaultId);
//         emit log_named_uint("xValue",xValue);

//         uint256 gain = plutus.checkExtract(vaultId);
//         emit log_named_uint("xValue",gain);
//     }
// }
