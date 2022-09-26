import { task, types } from "hardhat/config";
import { TaskArguments } from "hardhat/types";

task("Map:All")
  .addParam<string>(
    "routerChainid",
    "Remote ChainID (Router Specs)",
    "",
    types.string
  )
  .addParam<string>("actualChainid", "Remote ChainID", "", types.string)
  .setAction(async function (_taskArguments: TaskArguments, hre) {
    console.log("Mapping Plutus started");
    await hre.run("map:Plutus", {
      chainid: _taskArguments.routerChainid,
      nchainid: _taskArguments.actualChainid,
    });
    console.log("Mapping Plutus ended");

    console.log("Mapping StableCoin started");
    await hre.run("map:StableCoin", {
      chainid: _taskArguments.routerChainid,
      nchainid: _taskArguments.actualChainid,
    });
    console.log("Mapping StableCoin ended");
  });
