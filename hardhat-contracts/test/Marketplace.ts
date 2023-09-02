import { expect } from "chai";
import { ethers } from "hardhat";
import {
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { Marketplace } from "../typechain-types/contracts/Marketplace";

describe("Marketplace contract", function () {

  async function deploy() {
    const ToyNFT = await ethers.getContractFactory("ToyNFT");
    const Marketplace = await ethers.getContractFactory("Marketplace");
    const toyContract = await ToyNFT.deploy("NFT Marketplace test", "test");
    const marketplace = await Marketplace.deploy();

    const [owner, otherAccount] = await ethers.getSigners();

    return { toyContract, marketplace, owner, otherAccount };

  }

  async function deploy_listing() {
    const { toyContract, marketplace, owner, otherAccount } = await loadFixture(deploy);
    const mintPrice = await toyContract.MINT_PRICE();
    const url = "http://localhost/test";
    const tokenId = 1;

    // should increase eth balance of tnft and decrease eth balance of signer
    await expect(toyContract.mintTo(owner, url, { value: mintPrice })).to.changeEtherBalances([toyContract, owner], [mintPrice, -mintPrice]);
    // should set the tokenURI for tokenId 1
    expect(await toyContract.tokenURI(tokenId)).to.equal(url);

    // approve token id 1 for marketplace
    await toyContract.approve(marketplace, tokenId);
    const price = ethers.parseEther("0.5");

    // should list item at price
    await marketplace.listItem(toyContract, tokenId, price);

    return { toyContract, marketplace, owner, otherAccount };

  }

  describe("Listing", function () {
    it("should be able get listing", async function () {
      const { toyContract, marketplace, owner } = await loadFixture(deploy_listing);
      const price = ethers.parseEther("0.5");
      const tokenId = await toyContract.currentId();

      const listing: Marketplace.ListingStruct = await marketplace.getListing(toyContract, tokenId);

      expect(listing.price).to.equal(price);
      expect(listing.seller).to.equal(owner.address);
    });

    it("should not list unapproved items", async function () {
      const { toyContract, marketplace, otherAccount } = await loadFixture(deploy_listing);
      const mintPrice = await toyContract.MINT_PRICE();

      await toyContract.connect(otherAccount).mintTo(otherAccount, "test", { value: mintPrice });
      // should set the tokenURI for tokenId 1
      const tokenId = await toyContract.currentId();

      // cannot list unapproved item
      await expect(marketplace.listItem(toyContract, tokenId, ethers.parseEther("0.5"))).to.be.reverted;
    });

    it("should be able to cancel", async function () {
      const { toyContract, marketplace, owner } = await loadFixture(deploy_listing);
      const tokenId = await toyContract.currentId();

      await expect(marketplace.connect(owner).cancelListing(toyContract, tokenId))
        .to.emit(marketplace, "ItemCanceled")
        .withArgs(owner.address, toyContract.getAddress, tokenId);
    });

    it("should be able to update listing price", async function () {
      const { toyContract, marketplace, owner } = await loadFixture(deploy_listing);
      const tokenId = await toyContract.currentId();
      const price = ethers.parseEther("0.7");

      await expect(marketplace.connect(owner).updateListing(toyContract, tokenId, price))
        .to.emit(marketplace, "ItemListed")
        .withArgs(owner.address, toyContract.getAddress, tokenId, price);

      const listing: Marketplace.ListingStruct = await marketplace.getListing(toyContract, tokenId);
      expect(listing.price).to.equal(price);
    });

    it("should revert non owner update price", async function () {
      const { toyContract, marketplace, otherAccount } = await loadFixture(deploy_listing);
      const tokenId = await toyContract.currentId();
      const price = ethers.parseEther("0.7");

      await expect(marketplace.connect(otherAccount).updateListing(toyContract, tokenId, price))
        .to.be.revertedWithCustomError(marketplace, "NotOwner");
    });

    it("should transfer ownership of token when bought", async function () {
      const { toyContract, marketplace, otherAccount } = await loadFixture(deploy_listing);
      const tokenId = await toyContract.currentId();
      let listing: Marketplace.ListingStruct = await marketplace.getListing(toyContract, tokenId);

      // other account buys item
      await expect(marketplace.connect(otherAccount).buyItem(toyContract, tokenId, { value: listing.price }))
        .to.emit(marketplace, "ItemBought")
        .withArgs(otherAccount.address, toyContract.getAddress, tokenId, listing.price);

      // token should not be listed anymore
      listing = await marketplace.getListing(toyContract, tokenId);
      expect(listing.price).to.equal(0);
      expect(listing.seller).to.equal(ethers.ZeroAddress);

      const price = ethers.parseEther("0.7");
      // new owner of token approves marketplace and reslists the item 
      await toyContract.connect(otherAccount).approve(marketplace, tokenId);
      await marketplace.connect(otherAccount).listItem(toyContract, tokenId, price);

      listing = await marketplace.getListing(toyContract, tokenId);
      expect(listing.price).to.equal(price);
      expect(listing.seller).to.equal(otherAccount.address);
    });
  });
});