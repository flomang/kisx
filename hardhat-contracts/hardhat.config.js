/** @type import('hardhat/config').HardhatUserConfig */
require("@nomicfoundation/hardhat-toolbox");

module.exports = {
  solidity: "0.8.19",
  paths: {
    artifacts: "../svelte-waveportal-frontend/artifacts",
  },
  networks: {
    hardhat: {
      chainId: 1337,
    },
  },
};


