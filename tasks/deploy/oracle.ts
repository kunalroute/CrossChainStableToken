import { task, types } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("deploy:Oracle", "Deploys the oracle").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  console.log("Oracle Deployment Started:");

  const Oracle = await hre.ethers.getContractFactory("Oracle");
  const oracle = await Oracle.deploy();

  await oracle.deployed();

  await hre.run("STORE_DEPLOYMENTS", {
    contractName: "oracle",
    contractAddress: oracle.address,
  });

  console.log("Deployed oracle At:", oracle.address);

  return null;
});
