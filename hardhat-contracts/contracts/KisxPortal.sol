// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ERC721URIStorage, ERC721} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Counters} from "@openzeppelin/contracts/utils/Counters.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

// A contract that is ownable and allows users to buy and sell Lego sets as NFTs
contract KisxPortal is ERC721URIStorage, Ownable {
    using Counters for Counters.Counter;

    // a list of possible lot statuses
    enum LotStatus {
        None,
        OnSale,
        Sold,
        OffMarket
    }
    // a list of possible lot types
    enum LotType {
        None,
        Set,
        Minifig,
        Part,
        MOC,
        Box,
        Instruction,
        Other
    }

    // define a struct that will be used to store the lot information
    struct Lot {
        uint256 id;
        string title;
        string description;
        uint256 price;
        string date;
        address payable owner;
        LotStatus status;
        LotType lotType;
        string uri;
    }

    // define a struct that will be used to log the transaction
    struct LotTxn {
        uint256 lotId;
        uint256 price;
        address seller;
        address buyer;
        uint txnDate;
        LotStatus status;
    }

    // emit lot sold event
    event LotSold(
        uint _tokenId,
        string _title,
        uint256 _price,
        address _current_owner,
        address _buyer
    );

    // emit lot created event
    event LotCreate(
        uint _tokenId,
        string _title,
        string _description,
        uint256 _price,
        address _author
    );

    // emit lot cancelled event
    event LotCancel(uint _tokenId);

    // emit lot resell event
    event LotResell(uint _tokenId, uint _status, uint256 _price);

    // emit lot updated event
    event LotUpdated(Lot _updated);

    // emit balance withdrawn event
    event BalanceWithdrawn(address _payee, uint256 _balance);

    // emit unauthorized withdraw failure error
    error UnauthorizedWithdrawFailure();

    // tracks number of lots on sale and gets updated during minting(creation), buying and reselling
    Counters.Counter public pendingLotCount;
    // token index counter that gets updated during minting(creation)
    Counters.Counter public index;

    // mint price token
    uint256 public mintPrice;

    mapping(uint256 => LotTxn[]) private lotTxns;
    // uint256 public index; // uint256 value; is cheaper than uint256 value = 0;.
    Lot[] public lots;

    modifier validSender() {
        require(msg.sender != address(0), "Sender address must not be zero");
        _;
    }

    // this contract is an ERC721 non-fungible token
    // https://docs.openzeppelin.com/contracts/4.x/erc721
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
        uint256 _price,
        string memory _uri,
        LotType _lotType
    ) public payable validSender returns (uint256 tokenId) {
        require(bytes(_title).length > 0, "The title cannot be empty");
        require(bytes(_date).length > 0, "The date cannot be empty");
        require(
            bytes(_description).length > 0,
            "The description cannot be empty"
        );
        require(_price > 0, "The price cannot be 0");
        require(
            _lotType > LotType.None && _lotType <= LotType.Other,
            "The lot type cannot be None"
        );
        require(msg.value >= mintPrice, "The mint price is not paid");

        tokenId = index.current();
        Lot memory _lot = Lot({
            id: tokenId,
            title: _title,
            description: _description,
            price: _price,
            date: _date,
            owner: payable(msg.sender),
            status: LotStatus.OnSale,
            lotType: _lotType,
            uri: _uri
        });
        lots.push(_lot);
        _safeMint(msg.sender, tokenId);
        _setTokenURI(tokenId, _uri);

        emit LotCreate(tokenId, _title, _description, _price, msg.sender);

        // increment our token index counter
        index.increment();
        pendingLotCount.increment();

        // refund extra payment
        if (msg.value > mintPrice)
            payable(msg.sender).transfer(msg.value - mintPrice);
    }

    /* Pass the token ID and get the lot information */
    function findLot(uint256 _tokenId) public view returns (Lot memory) {
        return lots[_tokenId];
    }

    // buy the lot
    // ownership of lot is transferred to the buyer
    function buyLot(uint256 _tokenId) public payable validSender {
        Lot memory lot = findLot(_tokenId);
        // lot owner must not be zero
        require(lot.owner != address(0), "The owner cannot be zero");
        // buyer must not be the owner
        require(msg.sender != lot.owner, "You are the owner");
        // buyer must pay the price
        require(msg.value >= lot.price, "The price is not paid");
        // lot must be on sale
        require(lot.status == LotStatus.OnSale, "The lot is not for sale");

        // transfer ownership of tokenId from lot owner to sender
        _safeTransfer(lot.owner, msg.sender, _tokenId, "");
        //make a payment to the current owner
        lot.owner.transfer(lot.price);

        // log the transaction
        LotTxn memory _lotTxn = LotTxn({
            lotId: lot.id,
            price: lot.price,
            seller: lot.owner,
            buyer: msg.sender,
            txnDate: block.timestamp,
            status: LotStatus.Sold
        });
        lotTxns[lot.id].push(_lotTxn);

        pendingLotCount.decrement();
        emit LotSold(lot.id, lot.title, lot.price, lot.owner, msg.sender);

        // update the lot information
        lots[_tokenId].owner = payable(msg.sender);
        lots[_tokenId].status = LotStatus.OffMarket;

        //return extra payment
        if (msg.value > lot.price)
            payable(msg.sender).transfer(msg.value - lot.price);
    }

    // cancel the lot
    function cancelLot(uint256 _tokenId) public validSender {
        // contract owner can cancel the lot
        if (msg.sender != owner()) {
            // sender must own token
            require(isOwnerOf(_tokenId, msg.sender), "You are not the owner");
        }

        // lot must be on sale
        if (lots[_tokenId].status == LotStatus.OnSale) {
            lots[_tokenId].status = LotStatus.OffMarket;
            pendingLotCount.decrement();
            emit LotCancel(_tokenId);
        }
    }

    // resell the lot
    function updateLot(
        uint256 _tokenId,
        string memory _title,
        string memory _description,
        string memory _date,
        uint256 _price,
        string memory _uri,
        LotType _lotType,
        LotStatus _status
    ) public validSender {
        // must be the owner
        require(isOwnerOf(_tokenId, msg.sender), "You are not the owner");

        // lot type must be within the range
        require(
            _lotType <= LotType.Other && _status <= LotStatus.OffMarket,
            "param must be within the range"
        );

        Lot memory lot = findLot(_tokenId);

        if (lot.status == LotStatus.OnSale && _status == LotStatus.OffMarket) {
            pendingLotCount.decrement();
        } else if (
            lot.status == LotStatus.OffMarket && _status == LotStatus.OnSale
        ) {
            pendingLotCount.increment();
        }

        if (_status != LotStatus.None) {
            lot.status = _status;
        }
        if (_lotType != LotType.None) {
            lot.lotType = _lotType;
        }
        if (_price > 0) {
            lot.price = _price;
        }
        if (bytes(_title).length > 0) {
            lot.title = _title;
        }
        if (bytes(_description).length > 0) {
            lot.description = _description;
        }
        if (bytes(_date).length > 0) {
            lot.date = _date;
        }
        if (bytes(_uri).length > 0) {
            lot.uri = _uri;
        }

        lots[_tokenId] = lot;

        emit LotUpdated(lot);
    }

    // find all the lots that are on sale
    // return an array of token ID's
    function findAllPending() public view returns (uint256[] memory ids) {
        if (pendingLotCount.current() == 0) {
            return (new uint[](0));
        }

        uint256 arrLength = lots.length;
        ids = new uint256[](pendingLotCount.current());
        uint256 idx = 0;
        for (uint i = 0; i < arrLength; ++i) {
            Lot memory lot = lots[i];
            if (lot.status == LotStatus.OnSale) {
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

    // Only the account owner can withdraw the contract's balance.
    function withdrawBalance(address payable payee) external onlyOwner {
        uint256 balance = address(this).balance;
        (bool transferTx, ) = payee.call{value: balance}("");
        if (!transferTx) {
            revert UnauthorizedWithdrawFailure();
        }
        emit BalanceWithdrawn(payee, balance);
    }
}
