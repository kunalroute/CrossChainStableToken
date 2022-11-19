import "@nomiclabs/hardhat-waffle";
import "@nomiclabs/hardhat-etherscan";
import "@typechain/hardhat";
import "hardhat-gas-reporter";
import "solidity-coverage";
import "hardhat-deploy";
import "hardhat-deploy-ethers";
import "@openzeppelin/hardhat-upgrades";
import "./tasks/index";

import { resolve } from "path";

import { config as dotenvConfig } from "dotenv";
import { HardhatUserConfig } from "hardhat/config";
import { NetworkUserConfig } from "hardhat/types";

dotenvConfig({ path: resolve(__dirname, "./.env") });

const chainIds = {
  ganache: 1337,
  goerli: 5,
  hardhat: 31337,
  kovan: 42,
  mainnet: 1,
  rinkeby: 4,
  ropsten: 3,
  polygon: 137,
  mumbai: 80001,
  bnb: 56,
  ftm: 250,
};

// Ensure that we have all the environment variables we need.
const mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  throw new Error("Please set your MNEMONIC in a .env file");
}

const infuraApiKey: string | undefined = process.env.INFURA_API_KEY;
if (!infuraApiKey) {
  throw new Error("Please set your INFURA_API_KEY in a .env file");
}

const etherscanApiKey: string | undefined = process.env.ETHERSCAN_API_KEY;
if (!etherscanApiKey) {
  throw new Error("Please set your ETHERSCAN_API_KEY in a .env file");
}

function getChainConfig(network: keyof typeof chainIds): NetworkUserConfig {
  let url = "";
  url = "https://" + network + ".infura.io/v3/" + infuraApiKey;
  if (network == "polygon") {
    url = "https://polygon-rpc.com";
  }
  if (network == "mumbai") {
    url = "https://rpc-mumbai.matic.today";
  }
  if (network == "bnb") {
    url = "https://rpc.ankr.com/bsc";
  }
  if (network == "ftm") {
    url = "https://rpc.ftm.tools/";
  }

  return {
    accounts: [`${process.env.PRIVATE_KEY}`],
    chainId: chainIds[network],
    url,
    gasPrice: 170_000_000_000,
  };
}

const config: HardhatUserConfig = {
  defaultNetwork: "hardhat",
  gasReporter: {
    currency: "USD",
    enabled: process.env.REPORT_GAS ? true : false,
    excludeContracts: [],
    src: "./contracts",
  },
  networks: {
    hardhat: {
      accounts: {
        mnemonic,
      },
      chainId: chainIds.hardhat,
      // See https://github.com/sc-forks/solidity-coverage/issues/652
      hardfork: process.env.CODE_COVERAGE ? "berlin" : "london",
    },
    ganache1: {
      chainId: 7545,
      url: "https://rc-testnet1.routerprotocol.com/",
      accounts: [mnemonic],
    },
    ganache2: {
      chainId: 8545,
      url: "https://rc-testnet2.routerprotocol.com/",
      accounts: [mnemonic],
    },
    ganache3: {
      chainId: 6545,
      url: "https://rc-testnet3.routerprotocol.com/",
      accounts: [mnemonic],
    },
    goerli: getChainConfig("goerli"),
    kovan: getChainConfig("kovan"),
    rinkeby: getChainConfig("rinkeby"),
    ropsten: getChainConfig("ropsten"),
    polygon: getChainConfig("polygon"),
    mumbai: getChainConfig("mumbai"),
    bnb: getChainConfig("bnb"),
    ftm: getChainConfig("ftm"),
  },
  paths: {
    artifacts: "./artifacts",
    cache: "./cache",
    sources: "./contracts",
    tests: "./test",
  },
  etherscan: {
    apiKey: "SABG3ZEUPK117SQRRYI14W265HWCUPPSI6",
  },
  solidity: {
    compilers: [
      {
        version: "0.8.13",
        settings: {
          metadata: {
            // Not including the metadata hash
            // https://github.com/paulrberg/solidity-template/issues/31
            bytecodeHash: "none",
          },
          // Disable the optimizer when debugging
          // https://hardhat.org/hardhat-network/#solidity-optimizer-support
          optimizer: {
            enabled: true,
            runs: 10000,
          },
        },
      },
      {
        version: "0.6.11",
        settings: {
          optimizer: {
            enabled: true,
            runs: 10000,
          },
        },
      },
    ],
  },
  typechain: {
    outDir: "typechain",
    target: "ethers-v5",
  },
};

export default config;
