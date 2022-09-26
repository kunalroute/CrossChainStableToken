import { task } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("verify:Plutus").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  const deployment = require("../../deployments/deployments.json");
  const network = await hre.ethers.provider.getNetwork();
  const chainId = network.chainId;
  const contractAddress = deployment[chainId].plutus;
  await hre.run("verify:verify", {
    address: contractAddress,
    constructorArguments: [
      200, //Minimum Collateral Percentage
      "WETH PawnVault", //name of nft
      "WETHPV", //symbol of nft
      "hello plutus", //uri of nft
      deployment[chainId].stableCoin, //address of stablecoin
      deployment[chainId].collateral, //address of collateral
      deployment[chainId].treasury, //address of treasury
      deployment[chainId].oracle, //address of oracle
      deployment[chainId].genericHandler, //address of generichandler
    ],
  });
});
