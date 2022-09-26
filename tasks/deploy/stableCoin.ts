import { task, types } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("deploy:StableCoin", "Deploys the stable coin").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  //@ts-ignore
  const deployment = require("../../deployments/deployments.json");
  const network = await hre.ethers.provider.getNetwork();
  const chainId = network.chainId;
  const treasury = deployment[chainId].treasury;
  console.log("StableCoin Deployment Started:");
  const StableContract = await hre.ethers.getContractFactory("StableCoin");
  const StableCoin = await StableContract.deploy(
    "USDP",
    "USDP",
    deployment[chainId].genericHandler,
    deployment[chainId].feeToken,
    treasury
  );
  await StableCoin.deployed();

  await hre.run("STORE_DEPLOYMENTS", {
    contractName: "stableCoin",
    contractAddress: StableCoin.address,
  });

  console.log("Deployed StableCoin At:", StableCoin.address);
  return null;
});
