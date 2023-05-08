// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.10;

//import "solmate/tokens/ERC721.sol";
import {ERC721URIStorage, ERC721} from "openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {Strings} from "openzeppelin-contracts/contracts/utils/Strings.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";
import {Counters} from "openzeppelin-contracts/contracts/utils/Counters.sol";


error MintPriceNotPaid();
error MaxSupply();
error NonExistentTokenURI();
error WithdrawTransfer();

contract ToyNFT is ERC721URIStorage, Ownable {

    using Strings for uint256;
    using Counters for Counters.Counter;

    Counters.Counter public currentTokenId;
    uint256 public constant TOTAL_SUPPLY = 10_000;
    uint256 public constant MINT_PRICE = 0.08 ether;

    constructor(
        string memory _name,
        string memory _symbol
    ) ERC721(_name, _symbol) {
    }

    function mintTo(address recipient, string memory url) public payable returns (uint256) {
        if (recipient == address(0)) {
            revert("INVALID_RECIPIENT");
        }
        if (msg.value != MINT_PRICE) {
            revert MintPriceNotPaid();
        }
        currentTokenId.increment();
        uint256 newTokenId = currentTokenId.current();
        if (newTokenId > TOTAL_SUPPLY) {
            revert MaxSupply();
        }
        _safeMint(recipient, newTokenId);
        _setTokenURI(newTokenId, url);
        return newTokenId;
    }

    function withdrawPayments(address payable payee) external onlyOwner {
        uint256 balance = address(this).balance;
        (bool transferTx, ) = payee.call{value: balance}("");
        if (!transferTx) {
            revert WithdrawTransfer();
        }
    }
}
