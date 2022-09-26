import { deployments } from "hardhat";
import { task, types } from "hardhat/config";
import { TaskArguments } from "hardhat/types";
import { FEE_TOKEN, GENERIC_HANDLER } from "../constants";

task("deploy:Plutus", "Deploys the plutus").setAction(async function (
  taskArguments: TaskArguments,
  hre
) {
  console.log("Initial Setup Started:");
  const deployment = require("../../deployments/deployments.json");
  const network = await hre.ethers.provider.getNetwork();
  const chainId = network.chainId;

  const plutusContract = await hre.ethers.getContractFactory("Plutus");

  const minimumCollateralPercentage = 200;
  const name = "WETH PawnVault";
  const symbol = "WETHPV";
  const baseURI = "hello plutus";
  const _stableCoin = deployment[chainId].stableCoin;
  const _collateral = deployment[chainId].collateral;
  const _treasury = deployment[chainId].treasury;
  const OraclepriceSource = deployment[chainId].oracle;
  const genericHandlerAddress = deployment[chainId].genericHandler;

  const plutus = await plutusContract.deploy(
    minimumCollateralPercentage,
    name,
    symbol,
    baseURI,
    _stableCoin,
    _collateral,
    _treasury,
    OraclepriceSource,
    genericHandlerAddress
  );
  await plutus.deployed();

  await hre.run("STORE_DEPLOYMENTS", {
    contractName: "plutus",
    contractAddress: plutus.address,
  });

  console.log("Deployed Plutus At:", plutus.address);
  return null;
});
