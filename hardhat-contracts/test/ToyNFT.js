const { expect } = require("chai");

describe("NFT contract", function () {
  let tnft, signer, buyer, seller;

  before("deploy contract", async function () {
    const ToyNFT = await ethers.getContractFactory("ToyNFT");
    tnft = await ToyNFT.deploy("NFT test", "test");
    [signer, buyer, seller] = await ethers.getSigners();
  });

  it("should have the correct name and symbol", async function () {
    const name = await tnft.name();
    const symbol = await tnft.symbol();
    const owner = await tnft.owner();

    expect(owner).to.equal(signer.address);
    expect(name).to.equal("NFT test");
    expect(symbol).to.equal("test");

  });

  it("should revert when mint not paid", async function () {
    await expect(tnft.mintTo(signer, "http://localhost/testing/")).to.be.revertedWithCustomError(
      tnft,
      "MintPriceNotPaid"
    );
    expect(await tnft.MAX_SUPPLY()).to.equal(10000);
  });

  it("should revert for invalid signer", async function () {
    const mintPrice = await tnft.MINT_PRICE();
    // should revert 
    await expect(tnft.mintTo(ethers.ZeroAddress, "http://localhost/test", { value: mintPrice })).to.be.revertedWithCustomError(
      tnft,
      "InvalidRecipient"
    );
  });

  it("should mint an nft when the mint is paid", async function () {
    const mintPrice = await tnft.MINT_PRICE();
    const url = "http://localhost/test"; 

    // should increase eth balance of tnft and decrease eth balance of signer
    await expect(tnft.mintTo(signer, url, { value: mintPrice })).to.changeEtherBalances([tnft, signer], [mintPrice, -mintPrice]);
    // should set the tokenURI for tokenId 1
    expect(await tnft.tokenURI(1)).to.equal(url);
  });

  it("should be able to withdraw minting balance", async function () {
    const mintPrice = await tnft.MINT_PRICE();
    await expect(tnft.withdrawBalance(signer.address)).to.changeEtherBalance(signer, mintPrice);
  });

  it("should revert when withdraw not owner", async function () {
    const mintPrice = await tnft.MINT_PRICE();
    await expect(tnft.connect(buyer).withdrawBalance(buyer.address)).to.be.revertedWith("Ownable: caller is not the owner");
  });
});