# foundry-contracts  
![kisx](kisx.webp)

## Description

This is a [Foundry](ihttps://book.getfoundry.sh/) project for Ethereum smart contract development in Solidity. This project was initilized without git since it is a subproject of kisx. 

It's import that we remember this when installing new deps via forge. Example:

```
forge install --no-git openzeppelin/openzeppelin-contracts
```

Note: The vscode settings for this project are workspace specific. Refer to .vscode/settings.json file for vscode solidity settings - remappings, lib, etc. Without this special sauce vscode navigation to contracts won't work and you'll experience red dependency errors, but using forge from the command line will work just fine. 