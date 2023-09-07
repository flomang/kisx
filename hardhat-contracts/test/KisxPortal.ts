import { expect } from "chai";
import { ethers } from "hardhat";
import {
    loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { KisxPortal } from "../typechain-types/contracts/KisxPortal";

describe("Kisx Portal", function () {

    async function deploy() {
        const KisxPortal = await ethers.getContractFactory("KisxPortal");
        const mintPrice = ethers.parseEther("0003");
        const kisx = await KisxPortal.deploy("kisx", "ktest", mintPrice);
        const [owner, account1, account2] = await ethers.getSigners();

        return { kisx, owner, account1, account2, mintPrice };
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
            const { kisx, account1 } = await loadFixture(deploy);

            // expect revert from open zeppelin ownable
            await expect(kisx.connect(account1).setMintPrice(ethers.parseEther("0.1"))).to.be.revertedWith("Ownable: caller is not the owner");
        });

        it("should allow owner to set mint price", async function () {
            const { kisx, owner, account1 } = await loadFixture(deploy);
            const mintPrice = ethers.parseEther("0009");

            // expect revert from open zeppelin ownable
            await kisx.connect(owner).setMintPrice(mintPrice);

            const price = await kisx.mintPrice();
            expect(price).to.equal(mintPrice);
        });
    });

    describe("Minting", function () {
        it("should create a lot", async function () {
            const { kisx, owner, account1, mintPrice } = await loadFixture(deploy);
            const price = ethers.parseEther("3.5");

            await expect(kisx.connect(account1).createLot("Lego", "Lego Collection", "2023-09-05T22:27:54Z", price, "uri", 0, { value: mintPrice }))
                .to.emit(kisx, "LotCreate")
                .withArgs(0, "Lego", "Lego Collection", price, account1.address);;

            let pending = await kisx.findAllPending();
            let myLots = await kisx.connect(account1).findMyLots();

            expect(myLots.length).to.equal(pending.length);
            // both ids should match
            expect(myLots[0]).to.equal(pending[0]);

            await kisx.connect(account1).cancelLot(0);
            pending = await kisx.findAllPending();
            expect(pending.length).to.equal(0);
            myLots = await kisx.connect(account1).findMyLots();
            expect(myLots.length).to.equal(1);
        });

        it("should revert if cancel lot not owned", async function () {
            const { kisx, owner, account1, account2, mintPrice } = await loadFixture(deploy);
            const price = ethers.parseEther("3.5");

            await kisx.connect(account1).createLot("Lego", "Lego Collection", "2023-09-05T22:27:54Z", price, "uri", 0, { value: mintPrice })

            await expect(kisx.connect(account2).cancelLot(0)).to.be.revertedWith("You are not the owner");;
        });

        it("should allow owner to cancel lots", async function () {
            const { kisx, owner, account1, account2, mintPrice } = await loadFixture(deploy);
            const price = ethers.parseEther("3.5");

            await kisx.connect(account1).createLot("Lego", "Lego Collection", "2023-09-05T22:27:54Z", price, "uri", 0, { value: mintPrice })

            await expect(kisx.connect(owner).cancelLot(0)).to.emit(kisx, "LotCancel")
                .withArgs(0);

            let pending = await kisx.findAllPending();
            expect(pending.length).to.equal(0);

            let count = await kisx.pendingLotCount();
            expect(count).to.equal(0);
        });

        it("should allow a buyer to buy a lot", async function () {
            const { kisx, owner, account1, account2, mintPrice } = await loadFixture(deploy);
            const price = ethers.parseEther("3.5");

            await kisx.connect(account1).createLot("Lego", "Lego Collection", "2023-09-05T22:27:54Z", price, "uri", 0, { value: mintPrice })

            let lot: KisxPortal.LotStruct = await kisx.connect(account2).findLot(0);

            await expect(kisx.connect(account2).buyLot(0, { value: lot.price })).to.emit(kisx, "LotSold")
            let pending = await kisx.findAllPending();
            expect(pending.length).to.equal(0);

            let count = await kisx.pendingLotCount();
            expect(count).to.equal(0);

            await expect((await kisx.ownerOf(0))).to.equal(account2.address);
        });
    });
});