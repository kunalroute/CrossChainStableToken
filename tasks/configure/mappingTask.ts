import { Contract } from "ethers";
import { task, types } from "hardhat/config";

// chainid = Destination Chain IDs defined by Router. Eg: Polygon, Fantom and BSC are assigned chain IDs 1, 2, 3.
// nchainid = Actual Destination Chain IDs
task("map:Plutus", "Map Plutus Contracts")
  .addParam<string>(
    "chainid",
    "Remote ChainID (Router Specs)",
    "",
    types.string
  )
  .addParam<string>("nchainid", "Remote ChainID", "", types.string)
  .setAction(async (taskArgs, hre): Promise<null> => {
    const handlerABI = require("../../build/GenericHandler.json");

    const deployments = require("../../deployments/deployments.json");

    const network = await hre.ethers.provider.getNetwork();
    const lchainID = network.chainId.toString();

    const handlerContract: Contract = await hre.ethers.getContractAt(
      handlerABI,
      deployments[lchainID].genericHandler
    );

    await handlerContract.MapContract([
      deployments[lchainID].plutus,
      taskArgs.chainid,
      deployments[taskArgs.nchainid].plutus,
    ]);

    console.log("Plutus Mappings Done");
    return null;
  });

task("map:StableCoin", "Map StableCoin Contracts")
  .addParam<string>(
    "chainid",
    "Remote ChainID (Router Specs)",
    "",
    types.string
  )
  .addParam<string>("nchainid", "Remote ChainID", "", types.string)
  .setAction(async (taskArgs, hre): Promise<null> => {
    const handlerABI = require("../../build/GenericHandler.json");

    const deployments = require("../../deployments/deployments.json");

    const network = await hre.ethers.provider.getNetwork();
    const lchainID = network.chainId.toString();

    const handlerContract: Contract = await hre.ethers.getContractAt(
      handlerABI,
      deployments[lchainID].genericHandler
    );

    await handlerContract.MapContract([
      deployments[lchainID].stableCoin,
      taskArgs.chainid,
      deployments[taskArgs.nchainid].stableCoin,
    ]);

    console.log("Stable coin Mappings Done");
    return null;
  });

task("unmap:Plutus", "UnMap Plutus Contracts")
  .addParam<string>(
    "chainid",
    "Remote ChainID (Router Specs)",
    "",
    types.string
  )
  .addParam<string>("nchainid", "Remote ChainID", "", types.string)
  .setAction(async (taskArgs, hre): Promise<null> => {
    const handlerABI = require("../../build/GenericHandler.json");

    const deployments = require("../../deployments/deployments.json");

    const network = await hre.ethers.provider.getNetwork();
    const lchainID = network.chainId.toString();

    const handlerContract: Contract = await hre.ethers.getContractAt(
      handlerABI,
      deployments[lchainID].genericHandler
    );

    await handlerContract.UnMapContract([
      deployments[lchainID].plutus,
      taskArgs.chainid,
      deployments[taskArgs.nchainid].plutus,
    ]);

    console.log("Plutus Unmapping Done");
    return null;
  });

task("unmap:StableCoin", "UnMap StableCoin Contracts")
  .addParam<string>(
    "chainid",
    "Remote ChainID (Router Specs)",
    "",
    types.string
  )
  .addParam<string>("nchainid", "Remote ChainID", "", types.string)
  .setAction(async (taskArgs, hre): Promise<null> => {
    const handlerABI = require("../../build/GenericHandler.json");

    const deployments = require("../../deployments/deployments.json");

    const network = await hre.ethers.provider.getNetwork();
    const lchainID = network.chainId.toString();

    const handlerContract: Contract = await hre.ethers.getContractAt(
      handlerABI,
      deployments[lchainID].genericHandler
    );

    await handlerContract.UnMapContract([
      deployments[lchainID].stableCoin,
      taskArgs.chainid,
      deployments[taskArgs.nchainid].stableCoin,
    ]);

    console.log("Stable Coin Unmapping Done");
    return null;
  });
