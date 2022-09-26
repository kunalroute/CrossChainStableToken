import { task } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("deploy:Treasury", "Deploys the treasury").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  console.log("Treasury Deployment Started:");
  const treasuryContract = await hre.ethers.getContractFactory("Treasury");
  const treasury = await treasuryContract.deploy();
  await treasury.deployed();
  console.log("Deployed Treasury At:", treasury.address);

  await hre.run("STORE_DEPLOYMENTS", {
    contractName: "treasury",
    contractAddress: treasury.address,
  });

  return null;
});
