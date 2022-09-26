import { task } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("Deploy:All").setAction(async function (
  _taskArguments: TaskArguments,
  hre
) {
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
  console.log("Setting fee token started");
  await hre.run("SET_FEES_TOKEN");
  console.log("Setting fee token ended");
  console.log("Approving fee token started");
  await hre.run("APPROVE_FEES_TOKEN");
  console.log("Approving fee token ended");
  console.log("verifying treasury started");
  await hre.run("verify:Treasury");
  console.log("verifying treasury ended");
  console.log("verifying StableCoin started");
  await hre.run("verify:StableCoin");
  console.log("verifying StableCoin ended");
  console.log("verifying Plutus started");
  await hre.run("verify:Plutus");
  console.log("verifying Plutus ended");
});
