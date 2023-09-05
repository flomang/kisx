import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import '@typechain/hardhat'
import '@nomicfoundation/hardhat-ethers'
import '@nomicfoundation/hardhat-chai-matchers'

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      {
        version: "0.8.21",
      },
      {
        version: "0.8.19",
        settings: {},
      },
    ],
  },
  paths: {
    artifacts: "../svelte-waveportal-frontend/artifacts",
  },
  networks: {
    hardhat: {
      chainId: 1337,
    },
  },
};

export default config;
