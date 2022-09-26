import { task } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("verify:StableCoin").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  const deployment = require("../../deployments/deployments.json");
  const network = await hre.ethers.provider.getNetwork();
  const chainId = network.chainId;
  const contractAddress = deployment[chainId].stableCoin;
  await hre.run("verify:verify", {
    address: contractAddress,
    constructorArguments: [
      "USDP",
      "USDP",
      deployment[chainId].genericHandler,
      deployment[chainId].feeToken,
      deployment[chainId].treasury,
    ],
  });
});
