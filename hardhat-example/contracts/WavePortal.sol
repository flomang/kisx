// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract WavePortal {
    enum Reaction {
        Wave,
        Cake,
        Hype
    }

    struct Wave {
        Reaction reaction;
        string message;
        address waver;
        uint256 timestamp;
    }

    uint256 totalWaves;
    Wave[] public waveList;

    event NewWave(
        Reaction reaction,
        string message,
        address indexed from,
        uint256 timestamp
    );

    constructor() {}

    function wave(Reaction _reaction, string memory _message) public {
        totalWaves += 1;
        waveList.push(Wave(_reaction, _message, msg.sender, block.timestamp));
        emit NewWave(_reaction, _message, msg.sender, block.timestamp);
    }

    function getAllWaves() public view returns (Wave[] memory) {
        return waveList;
    }

    function getTotalWaves() public view returns (uint256) {
        return waveList.length;
    }
}