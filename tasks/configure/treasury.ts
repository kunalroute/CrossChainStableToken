import { task } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("whitelist:Plutus").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  console.log("Whitelisting Plutus");
  const deployment = require("../../deployments/deployments.json");
  const network = await hre.ethers.provider.getNetwork();
  const chainId = network.chainId;
  const treasuryContract = await hre.ethers.getContractFactory("Treasury");
  const treasury = await treasuryContract.attach(deployment[chainId].treasury);
  await treasury.authorize(deployment[chainId].plutus);
  console.log("Whitelisted Plutus");
});
