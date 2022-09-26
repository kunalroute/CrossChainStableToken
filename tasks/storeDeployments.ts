import { task, types } from "hardhat/config";
import fs from "fs";

task("STORE_DEPLOYMENTS", "store deployments")
  .addParam<string>("contractName", "Contract Name", "", types.string)
  .addParam<string>("contractAddress", "Contract Address", "", types.string)
  .setAction(async (taskArgs, { ethers }): Promise<null> => {
    const network = await ethers.provider.getNetwork();
    const networkID = network.chainId;

    const deployedContracts = require("../deployments/deployments.json");

    if (typeof deployedContracts[networkID] === "undefined") {
      deployedContracts[networkID] = {};
    }

    deployedContracts[networkID][taskArgs.contractName] =
      taskArgs.contractAddress;

    fs.writeFileSync(
      "deployments/deployments.json",
      JSON.stringify(deployedContracts)
    );

    return null;
  });
