// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AssetManagement {
    string public constant name = "DCAR Lending and Borrowing";
    address public owner;
    
    enum RiskTier { Low, Medium, High }
    
    struct Loan {
        address borrower;
        uint256 amount;
        RiskTier riskTier;
        bool repaid;
    }
    
    mapping(uint256 => Loan) public loans;
    uint256 public loanCount;

    event LoanCreated(uint256 loanId, address borrower, uint256 amount, RiskTier riskTier);
    event LoanRepaid(uint256 loanId);

    constructor() {
        owner = msg.sender;
    }

    function createLoan(uint256 _amount, RiskTier _riskTier) external {
        require(_amount > 0, "Amount must be greater than zero");
        
        loanCount++;
        loans[loanCount] = Loan(msg.sender, _amount, _riskTier, false);
        
        emit LoanCreated(loanCount, msg.sender, _amount, _riskTier);
    }

    function repayLoan(uint256 _loanId) external {
        Loan storage loan = loans[_loanId];
        require(msg.sender == loan.borrower, "Only the borrower can repay the loan");
        require(!loan.repaid, "Loan already repaid");
        
        loan.repaid = true;
        
        emit LoanRepaid(_loanId);
    }
}
