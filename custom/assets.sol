// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AssetDistribution {
    struct Tranche {
        string name;
        uint256 riskLevel;
        uint256 returnExpectation;
        uint256 totalAssets;
    }

    mapping(string => Tranche) public tranches;

    event TrancheCreated(string name, uint256 riskLevel, uint256 returnExpectation);

    function createTranche(string memory _name, uint256 _riskLevel, uint256 _returnExpectation) public {
        tranches[_name] = Tranche(_name, _riskLevel, _returnExpectation, 0);
        emit TrancheCreated(_name, _riskLevel, _returnExpectation);
    }

    function allocateAssets(string memory _trancheName, uint256 _amount) public {
        require(tranches[_trancheName].totalAssets + _amount >= tranches[_trancheName].totalAssets, "Overflow error");
        tranches[_trancheName].totalAssets += _amount;
    }

    function getTrancheDetails(string memory _trancheName) public view returns (string memory, uint256, uint256, uint256) {
        Tranche memory tranche = tranches[_trancheName];
        return (tranche.name, tranche.riskLevel, tranche.returnExpectation, tranche.totalAssets);
    }
}
