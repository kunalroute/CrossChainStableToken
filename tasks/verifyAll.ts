import { task } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("verify:All", "Verify the contracts").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  await hre.run("verify:Plutus");
  await hre.run("verify:Treasury");
  await hre.run("verify:Oracle");
  await hre.run("verify:StableCoin");
});
