import { expect } from "chai";
import { ethers } from "hardhat";
import {
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";

describe("Kisx Portal", function () {

  async function deploy() {
    const KisxPortal = await ethers.getContractFactory("KisxPortal");
    const mintPrice = ethers.parseEther("0003");
    const kisx = await KisxPortal.deploy("kisx", "ktest", mintPrice);
    const [owner, otherAccount] = await ethers.getSigners();

    return { kisx, owner, otherAccount, mintPrice };
  }

  describe("Deployment", function () {
    it("should have the correct name and symbol", async function () {
      const { kisx, owner, mintPrice } = await loadFixture(deploy);

      const name = await kisx.name();
      const symbol = await kisx.symbol();
      const o = await kisx.owner();
      const pending = await kisx.pendingLotCount();
      const price = await kisx.mintPrice();

      expect(o).to.equal(owner.address);
      expect(name).to.equal("kisx");
      expect(symbol).to.equal("ktest");
      expect(pending).to.equal(0);
      expect(price).to.equal(mintPrice);
    });

    it("should revert non owner set mint price", async function () {
        const { kisx, otherAccount } = await loadFixture(deploy);

        // expect revert from open zeppelin ownable
        await expect(kisx.connect(otherAccount).setMintPrice(ethers.parseEther("0.1"))).to.be.revertedWith("Ownable: caller is not the owner");
      });
  });
});