// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ERC721URIStorage, ERC721} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Counters} from "@openzeppelin/contracts/utils/Counters.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract KisxPortal is ERC721URIStorage, Ownable {
    using Counters for Counters.Counter;

    struct Lot {
        uint256 id;
        string title;
        string description;
        uint256 price;
        string date;
        string authorName;
        address payable author;
        address payable owner;
        // 1 means token has sale status (or still in selling) and 0 means token is already sold, ownership transferred and moved to off-market gallery
        uint status;
        string uri;
    }

    struct LotTxn {
        uint256 id;
        uint256 price;
        address seller;
        address buyer;
        uint txnDate;
        uint status;
    }

    // gets updated during minting(creation), buying and reselling
    Counters.Counter public pendingLotCount;
    Counters.Counter public index;

    // mint price token
    uint256 public mintPrice;

    mapping(uint256 => LotTxn[]) private lotTxns;
    // uint256 public index; // uint256 value; is cheaper than uint256 value = 0;.
    Lot[] public lots;

    // log events back to the user interface
    event LogSold(
        uint _tokenId,
        string _title,
        string _authorName,
        uint256 _price,
        address _author,
        address _current_owner,
        address _buyer
    );

    event LogCreate(
        uint _tokenId,
        string _title,
        string _category,
        string _authorName,
        uint256 _price,
        address _author
    );

    event LogCancel(uint _tokenId);

    event LogResell(uint _tokenId, uint _status, uint256 _price);

    constructor(
        string memory _name,
        string memory _symbol,
        uint256 _initialMintPrice
    ) ERC721(_name, _symbol) {
        mintPrice = _initialMintPrice;
    }

    function setMintPrice(uint256 _newMintPrice) external onlyOwner {
        mintPrice = _newMintPrice;
    }

    /* Create or minting the token */
    function createLot(
        string memory _title,
        string memory _description,
        string memory _date,
        string memory _authorName,
        uint256 _price,
        string memory _uri
    ) public payable returns (uint256 tokenId) {
        require(bytes(_title).length > 0, "The title cannot be empty");
        require(bytes(_date).length > 0, "The Date cannot be empty");
        require(
            bytes(_description).length > 0,
            "The description cannot be empty"
        );
        require(_price > 0, "The price cannot be empty");
        require(msg.value >= mintPrice, "The mint price is not paid");

        tokenId = index.current();
        Lot memory _lot = Lot({
            id: tokenId,
            title: _title,
            description: _description,
            price: _price,
            date: _date,
            authorName: _authorName,
            author: payable(msg.sender),
            owner: payable(msg.sender),
            status: 1,
            uri: _uri
        });
        lots.push(_lot);
        _safeMint(msg.sender, tokenId);
        _setTokenURI(tokenId, _uri);

        emit LogCreate(tokenId, _title, _date, _authorName, _price, msg.sender);
        index.increment();
        pendingLotCount.increment();

        // refund extra payment
        if (msg.value > mintPrice)
            payable(msg.sender).transfer(msg.value - mintPrice);
    }

    /* Pass the token ID and get the lot information */
    function findLot(
        uint256 _tokenId
    )
        public
        view
        returns (
            uint256,
            string memory,
            string memory,
            uint256,
            uint status,
            string memory,
            string memory,
            address,
            address payable,
            string memory
        )
    {
        Lot memory lot = lots[_tokenId];
        return (
            lot.id,
            lot.title,
            lot.description,
            lot.price,
            lot.status,
            lot.date,
            lot.authorName,
            lot.author,
            lot.owner,
            lot.uri
        );
    }

    function buyLot(uint256 _tokenId) public payable {
        (
            uint256 _id,
            string memory _title,
            ,
            uint256 _price,
            uint _status,
            ,
            string memory _authorName,
            address _author,
            address payable _current_owner,

        ) = findLot(_tokenId);
        require(_current_owner != address(0));
        require(msg.sender != address(0));
        require(msg.sender != _current_owner);
        require(msg.value >= _price);
        require(lots[_tokenId].owner != address(0));
        // transfer ownership of the art
        _safeTransfer(_current_owner, msg.sender, _tokenId, "");
        //return extra payment
        if (msg.value > _price)
            payable(msg.sender).transfer(msg.value - _price);
        //make a payment to the current owner
        _current_owner.transfer(_price);
        lots[_tokenId].owner = payable(msg.sender);
        lots[_tokenId].status = 0;

        LotTxn memory _lotTxn = LotTxn({
            id: _id,
            price: _price,
            seller: _current_owner,
            buyer: msg.sender,
            txnDate: block.timestamp,
            status: _status
        });
        lotTxns[_id].push(_lotTxn);
        pendingLotCount.decrement();
        emit LogSold(
            _tokenId,
            _title,
            _authorName,
            _price,
            _author,
            _current_owner,
            msg.sender
        );
    }

    function cancelLot(uint256 _tokenId) public payable {
        require(msg.sender != address(0));
        if (msg.sender != owner()) {
            require(isOwnerOf(_tokenId, msg.sender), "You are not the owner");
        }
        lots[_tokenId].status = 0;
        pendingLotCount.decrement();
        emit LogCancel(_tokenId);
    }

    function resellLot(uint256 _tokenId, uint256 _price) public payable {
        require(msg.sender != address(0));
        require(isOwnerOf(_tokenId, msg.sender), "You are not the owner");
        lots[_tokenId].status = 1;
        lots[_tokenId].price = _price;
        pendingLotCount.increment();
        emit LogResell(_tokenId, 1, _price);
    }

    function findAllPending() public view returns (uint256[] memory ids) {
        if (pendingLotCount.current() == 0) {
            return (new uint[](0));
        }

        uint256 arrLength = lots.length;
        ids = new uint256[](pendingLotCount.current());
        uint256 idx = 0;
        for (uint i = 0; i < arrLength; ++i) {
            Lot memory lot = lots[i];
            if (lot.status == 1) {
                ids[idx] = lot.id;
                idx++;
            }
        }
    }

    /* Return the token ID's that belong to the caller */
    function findMyLots() public view returns (uint256[] memory myLots) {
        require(msg.sender != address(0));
        uint256 numOftokens = balanceOf(msg.sender);
        if (numOftokens == 0) {
            return new uint256[](0);
        } else {
            myLots = new uint256[](numOftokens);
            uint256 idx = 0;
            uint256 arrLength = lots.length;
            for (uint256 i = 0; i < arrLength; i++) {
                if (ownerOf(i) == msg.sender) {
                    myLots[idx] = i;
                    idx++;
                }
            }
        }
    }

    /* return true if the address is the owner of the token or else false */
    function isOwnerOf(
        uint256 tokenId,
        address account
    ) public view returns (bool) {
        address owner = ownerOf(tokenId);
        require(owner != address(0));
        return owner == account;
    }
}
