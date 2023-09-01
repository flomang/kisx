import { expect } from "chai";
import { ethers } from "hardhat";
import {
  time,
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { Marketplace } from "../typechain-types/contracts/Marketplace"; // Adjust the path as needed

describe("Marketplace contract", function () {

  async function deploy() {
    const ToyNFT = await ethers.getContractFactory("ToyNFT");
    const Marketplace = await ethers.getContractFactory("Marketplace");
    const toyContract = await ToyNFT.deploy("NFT Marketplace test", "test");
    const marketplace = await Marketplace.deploy();

    const [owner, otherAccount] = await ethers.getSigners();

    return { toyContract, marketplace, owner, otherAccount };

  }

  describe("Listing", function () {
    it("should be able to list an approved item", async function () {
      const { toyContract, marketplace, owner, otherAccount } = await loadFixture(deploy);
      const mintPrice = await toyContract.MINT_PRICE();
      const url = "http://localhost/test";

      // should increase eth balance of tnft and decrease eth balance of signer
      await expect(toyContract.mintTo(owner, url, { value: mintPrice })).to.changeEtherBalances([toyContract, owner], [mintPrice, -mintPrice]);
      // should set the tokenURI for tokenId 1
      expect(await toyContract.tokenURI(1)).to.equal(url);

      // approve token id 1 for marketplace
      const response = await toyContract.approve(marketplace, 1);

      await marketplace.listItem(toyContract, 1, ethers.parseEther("0.5"));
      const listing: Marketplace.ListingStruct = await marketplace.getListing(toyContract, 1);

      expect(listing.price).to.equal(ethers.parseEther("0.5"));
      expect(listing.seller).to.equal(owner.address);
    });

    it("should not list unapproved items", async function () {
      const { toyContract, marketplace, owner, otherAccount } = await loadFixture(deploy);
      const mintPrice = await toyContract.MINT_PRICE();
      const url = "http://localhost/test";

      // should increase eth balance of tnft and decrease eth balance of signer
      await expect(toyContract.mintTo(owner, url, { value: mintPrice })).to.changeEtherBalances([toyContract, owner], [mintPrice, -mintPrice]);
      // should set the tokenURI for tokenId 1
      expect(await toyContract.tokenURI(1)).to.equal(url);

      // cannot list unapproved item
      await expect(marketplace.listItem(toyContract, 1, ethers.parseEther("0.5"))).to.be.reverted;
    });
  });
});