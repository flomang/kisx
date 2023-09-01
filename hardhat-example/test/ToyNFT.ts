import { expect } from "chai";
import { ethers } from "hardhat";
import {
  time,
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";

describe("NFT contract", function () {

  async function deploy() {
    const ToyNFT = await ethers.getContractFactory("ToyNFT");
    const toyContract = await ToyNFT.deploy("NFT test", "test");
    const [owner, otherAccount] = await ethers.getSigners();

    //[signer, buyer, seller] = await ethers.getSigners();
    return { toyContract, owner, otherAccount };

  }

  describe("Deployment", function () {
    it("should have the correct name and symbol", async function () {
      const { toyContract, owner } = await loadFixture(deploy);

      const name = await toyContract.name();
      const symbol = await toyContract.symbol();
      const o = await toyContract.owner();

      expect(o).to.equal(owner.address);
      expect(name).to.equal("NFT test");
      expect(symbol).to.equal("test");
      expect(await toyContract.MAX_SUPPLY()).to.equal(10000);
    });

    it("should revert when mint not paid", async function () {
      const { toyContract, otherAccount } = await loadFixture(deploy);

      await expect(toyContract.connect(otherAccount).mintTo(otherAccount, "http://localhost/testing/")).to.be.revertedWithCustomError(
        toyContract,
        "MintPriceNotPaid"
      );
    });

    it("should revert for invalid signer", async function () {
      const { toyContract, owner, otherAccount } = await loadFixture(deploy);
      const mintPrice = await toyContract.MINT_PRICE();
      // should revert 
      await expect(toyContract.connect(otherAccount).mintTo(ethers.ZeroAddress, "http://localhost/test", { value: mintPrice })).to.be.revertedWithCustomError(
        toyContract,
        "InvalidRecipient"
      );
    });

    it("should mint an nft when the mint is paid", async function () {
      const { toyContract, owner, otherAccount } = await loadFixture(deploy);
      const mintPrice = await toyContract.MINT_PRICE();
      const url = "http://localhost/test";

      // should increase eth balance of tnft and decrease eth balance of signer
      await expect(toyContract.mintTo(owner, url, { value: mintPrice })).to.changeEtherBalances([toyContract, owner], [mintPrice, -mintPrice]);
      // should set the tokenURI for tokenId 1
      expect(await toyContract.tokenURI(1)).to.equal(url);
    });
  });

  describe("Withdraw", function () {
    it("should be able to withdraw minting balance", async function () {
      const { toyContract, owner, otherAccount } = await loadFixture(deploy);
      const mintPrice = await toyContract.MINT_PRICE();
      const url = "http://localhost/test";

      // should increase eth balance of tnft and decrease eth balance of signer
      await expect(toyContract.mintTo(owner, url, { value: mintPrice })).to.changeEtherBalances([toyContract, owner], [mintPrice, -mintPrice]);
      // should set the tokenURI for tokenId 1
      expect(await toyContract.tokenURI(1)).to.equal(url);
      await expect(toyContract.withdrawBalance(owner.address)).to.changeEtherBalance(owner, mintPrice);
    });

    it("should revert when withdraw not owner", async function () {
      const { toyContract, owner, otherAccount } = await loadFixture(deploy);
      const mintPrice = await toyContract.MINT_PRICE();
      await expect(toyContract.connect(otherAccount).withdrawBalance(otherAccount.address)).to.be.revertedWith("Ownable: caller is not the owner");
    });
  });
});