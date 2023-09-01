// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.10;

import {ERC721URIStorage, ERC721} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Counters} from "@openzeppelin/contracts/utils/Counters.sol";

error InvalidRecipient(address);
error MintPriceNotPaid(uint256);
error MaxSupply();
error NonExistentTokenURI();
error WithdrawTransferFailure();

// ERC721URIStorage is an extension of the ERC721 standard that adds a tokenURI to points
// to the item's metadata. 
contract ToyNFT is ERC721URIStorage, Ownable {

    using Strings for uint256;
    using Counters for Counters.Counter;

    Counters.Counter public currentTokenId;
    uint256 public constant MAX_SUPPLY = 10_000;
    uint256 public constant MINT_PRICE = 0.08 ether;

    constructor(
        string memory _name,
        string memory _symbol
    ) ERC721(_name, _symbol) {
    }

    // Mints a new token and assigns it to an address.
    // The token's URI is set to the provided URL.
    function mintTo(address recipient, string memory url) public payable returns (uint256) {
        if (recipient == address(0)) {
            revert InvalidRecipient(recipient);
        }
        if (msg.value != MINT_PRICE) {
            revert MintPriceNotPaid(MINT_PRICE);
        }
        currentTokenId.increment();
        uint256 newTokenId = currentTokenId.current();
        if (newTokenId > MAX_SUPPLY) {
            revert MaxSupply();
        }

        _safeMint(recipient, newTokenId);
        // Set the tokenURI (ERC721URIStorage) to the provided URL
        _setTokenURI(newTokenId, url);
        return newTokenId;
    }

    // Only the account owner can withdraw the contract's balance.
    function withdrawBalance(address payable payee) external onlyOwner {
        uint256 balance = address(this).balance;
        (bool transferTx, ) = payee.call{value: balance}("");
        if (!transferTx) {
            revert WithdrawTransferFailure();
        }
    }
}
