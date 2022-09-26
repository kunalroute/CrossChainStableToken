import { task, types } from "hardhat/config";

task("SET_FEES_TOKEN", "Sets the fee token address").setAction(
  async (taskArgs, hre): Promise<null> => {
    const deployment = require("../../deployments/deployments.json");
    const network = await hre.ethers.provider.getNetwork();
    const chainId = network.chainId;
    const PlutusAddress = deployment[chainId].plutus;
    const FeeTokenAddress = deployment[chainId].feeToken;
    const Plutus = await hre.ethers.getContractFactory("Plutus");
    const plutus = await Plutus.attach(PlutusAddress);
    await plutus.setFeesToken(FeeTokenAddress);

    console.log(`Fee token address set`);
    return null;
  }
);
