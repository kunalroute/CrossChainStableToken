import { task, types } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("deploy:Collateral", "Deploys the collateral").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  console.log("Collateral Deployment Started:");

  const Collateral = await hre.ethers.getContractFactory("Collateral");
  const collateral = await Collateral.deploy("Collateral Token", "CTK");

  await collateral.deployed();

  await hre.run("STORE_DEPLOYMENTS", {
    contractName: "collateral",
    contractAddress: collateral.address,
  });

  console.log("Deployed collateral At:", collateral.address);

  return null;
});
