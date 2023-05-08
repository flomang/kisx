// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {ToyNFT} from "src/ToyNFT.sol";
import {Marketplace} from "src/Marketplace.sol";

contract MarketPlaceTest is Test {
    Marketplace public marketPlace;
    ToyNFT public toyFactory;

    address admin = address(0x3);
    address seller = address(0x6);
    address buyer = address(0x9);

    //event DogMinted(uint256 indexed tokenId);

    function setUp() public {
        vm.deal(seller, 1 ether);
        vm.deal(buyer, 100.5 ether);

        vm.startPrank(admin);
        marketPlace = new Marketplace();
        toyFactory = new ToyNFT("Deez NFTs", "DEEZ-NFTS");
        vm.stopPrank();
    }

    function test_ListItem() public {
        vm.startPrank(seller);
        uint256 tokenId = toyFactory.mintTo{value: 0.08 ether}(address(seller), "my_url_to_meta_data");

        // approve marketplace to sell nft and transfer ownership
        toyFactory.approve(address(marketPlace), tokenId);
        marketPlace.listItem(address(toyFactory), tokenId, 0.5 ether);
        vm.stopPrank();
        console.log(toyFactory.tokenURI(tokenId));

        Marketplace.Listing memory listing = marketPlace.getListing(address(toyFactory), tokenId);
        assertEq(listing.price, 0.5 ether);
        assertEq(listing.seller, seller);
    }
}
