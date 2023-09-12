import { expect } from "chai";
import { ethers } from "hardhat";
import {
    loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { KisxPortal } from "../typechain-types/contracts/KisxPortal";
import exp from "constants";

describe("Kisx Portal", function () {

    async function deploy() {
        const KisxPortal = await ethers.getContractFactory("KisxPortal");
        const mintPrice = ethers.parseEther("0003");
        const kisx = await KisxPortal.deploy("kisx", "ktest", mintPrice);

        const [owner, account1, account2] = await ethers.getSigners();

        const price = ethers.parseEther("3.5");

        // create a test lot set (type: 1) with account1, it will be assigned token ID 0
        await kisx.connect(account1).createLot("Lego test", "Lego test Collection", "2023-09-05T22:27:54Z", price, "uri", 1, { value: mintPrice });

        return { kisx, owner, account1, account2, mintPrice };
    }


    describe("General", function () {
        it("should have the correct name, symbol, and mint price", async function () {
            const { kisx, owner, mintPrice } = await loadFixture(deploy);

            // read the contract's name, symbol, owner, and mint price
            const name = await kisx.name();
            const symbol = await kisx.symbol();
            const contractOwner = await kisx.owner();
            const pending = await kisx.pendingLotCount();
            const price = await kisx.mintPrice();

            // expect owner to be the deployer
            expect(contractOwner).to.equal(owner.address);
            expect(name).to.equal("kisx");
            expect(symbol).to.equal("ktest");
            expect(pending).to.equal(1);
            expect(price).to.equal(mintPrice);
        });
    });

    describe("Contract Owner", function () {
        it("should revert non owner set mint price", async function () {
            const { kisx, account1 } = await loadFixture(deploy);

            // expect revert from open zeppelin ownable
            await expect(kisx.connect(account1).setMintPrice(ethers.parseEther("0.1"))).to.be.revertedWith("Ownable: caller is not the owner");
        });

        it("should allow owner to set mint price", async function () {
            const { kisx, owner } = await loadFixture(deploy);
            const mintPrice = ethers.parseEther("0009");

            await kisx.connect(owner).setMintPrice(mintPrice);

            const price = await kisx.mintPrice();
            expect(price).to.equal(mintPrice);
        });

        it("should revert non contract owner withdraw", async function () {
            const { kisx, account1 } = await loadFixture(deploy);

            // expect revert from open zeppelin ownable
            await expect(kisx.connect(account1).withdrawBalance(account1.address)).to.be.revertedWith("Ownable: caller is not the owner");
        });

        it("should allow the contract owner to withdraw", async function () {
            const { kisx, owner, mintPrice } = await loadFixture(deploy);

            // expect withdraw event with owner address and mintPrice balance
            await expect(kisx.connect(owner).withdrawBalance(owner.address)).to.emit(kisx, "BalanceWithdrawn").withArgs(owner.address, mintPrice);
        });
    });

    describe("Selling", function () {
        it("should allow users to mint an NFT", async function () {
            const { kisx, account1, mintPrice } = await loadFixture(deploy);
            const price = ethers.parseEther("3.5");
            const tokenId = 1;

            await expect(kisx.connect(account1).createLot("Lego", "Lego Collection", "2023-09-05T22:27:54Z", price, "uri", 1, { value: mintPrice }))
                .to.emit(kisx, "LotCreate")
                .withArgs(tokenId, "Lego", "Lego Collection", price, account1.address);;

            let pending = await kisx.findAllPending();
            let myLots = await kisx.connect(account1).findMyLots();

            expect(myLots.length).to.equal(pending.length);
            // both ids should match
            expect(myLots[0]).to.equal(pending[0]);

            await kisx.connect(account1).cancelLot(0);
            pending = await kisx.findAllPending();
            expect(pending.length).to.equal(1);
            myLots = await kisx.connect(account1).findMyLots();
            expect(myLots.length).to.equal(2);
        });

        it("should revert if NFT details are missing", async function () {
            const { kisx, account1, mintPrice } = await loadFixture(deploy);
            const price = ethers.parseEther("3.5");
            const tokenId = 1;

            await expect(kisx.connect(account1).createLot("", "", "", 0, "", 0, { value: mintPrice }))
                .to.be.revertedWith("The title cannot be empty");

            await expect(kisx.connect(account1).createLot("Test", "", "", 0, "", 0, { value: mintPrice }))
                .to.be.revertedWith("The date cannot be empty");

            await expect(kisx.connect(account1).createLot("Test", "", "2023-09-05T22:27:54Z", 0, "", 0, { value: mintPrice }))
                .to.be.revertedWith("The description cannot be empty");

            await expect(kisx.connect(account1).createLot("Test", "Description", "2023-09-05T22:27:54Z", 0, "", 0, { value: mintPrice }))
                .to.be.revertedWith("The price cannot be 0");
            
            await expect(kisx.connect(account1).createLot("Test", "Description", "2023-09-05T22:27:54Z", price, "", 0, { value: mintPrice }))
                .to.be.revertedWith("The lot type cannot be None");
        });
    });

    describe("Canceling", function () {
        it("should revert a cancel if the address doesn't own the token ID", async function () {
            const { kisx, account2 } = await loadFixture(deploy);
            const tokenId = 0;

            await expect(kisx.connect(account2).cancelLot(tokenId)).to.be.revertedWith("You are not the owner");;
        });

        it("should allow contract owner to cancel", async function () {
            const { kisx, owner } = await loadFixture(deploy);
            const tokenId = 0;

            await expect(kisx.connect(owner).cancelLot(tokenId)).to.emit(kisx, "LotCancel")
                .withArgs(tokenId);
        });

        it("should allow token ID owner to cancel", async function () {
            const { kisx, account1 } = await loadFixture(deploy);
            const tokenId = 0;

            await expect(kisx.connect(account1).cancelLot(tokenId)).to.emit(kisx, "LotCancel")
                .withArgs(tokenId);

            let pending = await kisx.findAllPending();
            expect(pending.length).to.equal(0);

            let count = await kisx.pendingLotCount();
            expect(count).to.equal(0);

            let lot: KisxPortal.LotStruct = await kisx.connect(account1).findLot(0);

            // expect lot status to be 3 (OffMarket)
            expect(lot.status).to.equal(3)
        });
    });

    describe("Buying", function () {
        it("should allow a buyer to buy", async function () {
            const { kisx, account2 } = await loadFixture(deploy);

            let lot: KisxPortal.LotStruct = await kisx.connect(account2).findLot(0);

            // expect lot sold event
            await expect(kisx.connect(account2).buyLot(0, { value: lot.price })).to.emit(kisx, "LotSold")
            let pending = await kisx.findAllPending();
            expect(pending.length).to.equal(0);

            let count = await kisx.pendingLotCount();
            expect(count).to.equal(0);

            await expect((await kisx.ownerOf(0))).to.equal(account2.address);
        });

        it("should revert when the buyer attempts to buy an item off market", async function () {
            const { kisx, owner, account2 } = await loadFixture(deploy);

            let lot: KisxPortal.LotStruct = await kisx.connect(account2).findLot(0);

            // expect lot sold event
            await expect(kisx.connect(account2).buyLot(0, { value: lot.price })).to.emit(kisx, "LotSold")
            await expect(kisx.connect(owner).buyLot(0, { value: lot.price })).to.be.revertedWith("The lot is not for sale");
        });


        it("should revert when the buyer doesn't pay", async function () {
            const { kisx, account2 } = await loadFixture(deploy);
            // expect revert when buyer doesn't pay 
            await expect(kisx.connect(account2).buyLot(0, { value: 0 })).to.be.revertedWith("The price is not paid");
        });

        it("should revert when the buyer attempts to buy from himself", async function () {
            const { kisx, account1, account2 } = await loadFixture(deploy);

            let lot: KisxPortal.LotStruct = await kisx.connect(account2).findLot(0);

            // expect revert when buyer doesn't pay 
            await expect(kisx.connect(account1).buyLot(0, { value: lot.price })).to.be.revertedWith("You are the owner");
        });
    });

    describe("Updating", function () {
        it("should allow an update", async function () {
            const { kisx, account1 } = await loadFixture(deploy);
            const tokenId = 0;
            let none = 0;

            let lotOriginal: KisxPortal.LotStruct = await kisx.connect(account1).findLot(tokenId);
            await expect(kisx.connect(account1).updateLot(tokenId, "Lego updated set", "", "", 0, "", none, none)).to.emit(kisx, "LotUpdated");

            let lotUpdated = await kisx.connect(account1).findLot(tokenId);
            // updated title
            expect(lotUpdated.title).to.equal("Lego updated set");
            // none of these should have changed
            expect(lotUpdated.date).to.equal(lotOriginal.date);
            expect(lotUpdated.description).to.equal(lotOriginal.description);
            expect(lotUpdated.price).to.equal(lotOriginal.price);
            expect(lotUpdated.uri).to.equal(lotOriginal.uri);
            expect(lotUpdated.lotType).to.equal(lotOriginal.lotType);
            expect(lotUpdated.status).to.equal(lotOriginal.status);

            await expect(kisx.connect(account1).updateLot(tokenId, "", "", "", 0, "", none, 3)).to.emit(kisx, "LotUpdated");
            let lotUpdated2 = await kisx.connect(account1).findLot(tokenId);
            // should be off market status 
            expect(lotUpdated2.status).to.equal(3);

            let pending = await kisx.findAllPending();
            let count = await kisx.pendingLotCount();
            // 0 pending
            expect(pending.length).to.equal(0);
            expect(count).to.equal(0);
        });

        it("should revert an update on non owned", async function () {
            const { kisx, account2 } = await loadFixture(deploy);
            const tokenId = 0;
            let none = 0;

            let lot: KisxPortal.LotStruct = await kisx.connect(account2).findLot(0);

            // expect lot sold event
            await expect(kisx.connect(account2).updateLot(tokenId, "Lego updated set", "", "", 0, "", none, none)).to.be.revertedWith("You are not the owner");
           
        });

        it("should revert an invalid status and type", async function () {
            const { kisx, account1 } = await loadFixture(deploy);
            const tokenId = 0;
            let none = 0;

            // expect revert 
            await expect(kisx.connect(account1).updateLot(tokenId, "Lego updated set", "", "", 0, "", 8, none)).to.be.reverted;
            await expect(kisx.connect(account1).updateLot(tokenId, "Lego updated set", "", "", 0, "", none, 8)).to.be.reverted;
           
        });
    });
});