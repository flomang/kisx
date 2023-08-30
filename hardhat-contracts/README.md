# hardhat ethereum contracts 

Powered by [`hardhat`](https://hardhat.org/tutorial).

## How the project was created
```bash
mkdir hardhat-contracts
cd hardhat-contracts
yarn init
yarn add --dev hardhat
yarn add --dev @nomicfoundation/hardhat-toolbox @nomicfoundation/hardhat-network-helpers @nomicfoundation/hardhat-chai-matchers @nomicfoundation/hardhat-ethers @nomicfoundation/hardhat-verify chai ethers hardhat-gas-reporter solidity-coverage @typechain/hardhat typechain @typechain/ethers-v6
``` 

refer to [`hardhat`](https://hardhat.org/tutorial/creating-a-new-hardhat-project).


## Compiling contracts 
```bash
npx hardhat compile
```

## Running a hardhat node
```bash
npx hardhat node 
```

## Deploying 

To create a production version of your app:

```bash
npx hardhat run scripts/deploy.ts --network localhost
```
