import { task } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("Deploy:All").setAction(async function (
  _taskArguments: TaskArguments,
  hre
) {
  console.log("Deploying oracle started");
  await hre.run("deploy:Oracle");
  console.log("Deploying oracle ended");

  console.log("Deploying collateral started");
  await hre.run("deploy:Collateral");
  console.log("Deploying collateral ended");

  console.log("Deploying treasury started");
  await hre.run("deploy:Treasury");
  console.log("Deploying treasury ended");

  console.log("Deploying stablecoin started");
  await hre.run("deploy:StableCoin");
  console.log("Deploying stablecoin ended");

  console.log("Deploying Plutus started");
  await hre.run("deploy:Plutus");
  console.log("Deploying Plutus ended");

  console.log("Whitelisting Plutus on treasury started");
  await hre.run("whitelist:Plutus");
  console.log("Whitelisting Plutus on treasury ended");

  // console.log("verifying contracts started");
  // await hre.run("verify:All");
  // console.log("verifying contracts ended");
});
