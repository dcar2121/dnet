# --- Glossary Terms as Constants ---
# Entity-Relationship (ER) Model Terms
class ERModel:
    USERS = "Individuals/entities performing transactions"
    ASSETS = "Financial instruments supporting tranches (name, type, value)"
    TRANCHES = "Segments of assets categorized by risk, interest rate, total value"
    TRANSACTIONS = "User actions: deposits, withdrawals, rebalancing (amount, date)"
    VALIDATION_METRIC = "Metric to evaluate transaction validity"

# --- Asset Class ---
class Asset:
    def __init__(self, name, asset_type, value):
        self.name = name
        self.asset_type = asset_type
        self.value = value

    def __str__(self):
        return f"{self.name} ({self.asset_type}) - Value: {self.value}"

# --- Tranche Class ---
class Tranche:
    def __init__(self, name, risk_level, interest_rate, total_value):
        self.name = name
        self.risk_level = risk_level
        self.interest_rate = interest_rate
        self.total_value = total_value

    def __str__(self):
        return f"{self.name} ({self.risk_level} risk, {self.interest_rate}% interest, Total Value: {self.total_value})"

# --- Transaction Class ---
class Transaction:
    def __init__(self, user_id, asset, amount, transaction_type, date):
        self.user_id = user_id
        self.asset = asset
        self.amount = amount
        self.transaction_type = transaction_type
        self.date = date

    def __str__(self):
        return f"User {self.user_id} - {self.transaction_type} {self.amount} units of {self.asset.name} on {self.date}"

# --- Validation Engine Class ---
class ValidationEngine:
    def __init__(self, assets, tranches):
        self.assets = assets
        self.tranches = tranches

    def validate_transaction(self, transaction):
        # Simple validation logic
        if transaction.amount <= 0:
            return False, f"Invalid transaction amount: {transaction.amount}"
        
        # Check if the asset exists and is valid
        if transaction.asset not in self.assets:
            return False, f"Asset {transaction.asset.name} does not exist"
        
        # Check if the transaction type is valid
        if transaction.transaction_type not in ["deposit", "withdrawal", "rebalancing"]:
            return False, f"Invalid transaction type: {transaction.transaction_type}"
        
        # Additional validation logic can be added here
        return True, ""

# --- Example Usage ---
if __name__ == "__main__":
    # Create some assets
    asset1 = Asset("Stocks", "Equity", 10000)
    asset2 = Asset("Bonds", "Debt", 5000)

    # Create some tranches
    tranche1 = Tranche("Low Risk", "Low", 2, 50000)
    tranche2 = Tranche("High Risk", "High", 5, 100000)

    # Create a validation engine
    validation_engine = ValidationEngine([asset1, asset2], [tranche1, tranche2])

    # Create some transactions
    transaction1 = Transaction("User1", asset1, 500, "deposit", "2022-01-01")
    transaction2 = Transaction("User2", asset2, 200, "withdrawal", "2022-01-02")

    # Validate transactions
    valid, reason = validation_engine.validate_transaction(transaction1)
    print(f"Transaction {transaction1} is {'valid' if valid else 'invalid'}. Reason: {reason}")

    valid, reason = validation_engine.validate_transaction(transaction2)
    print(f"Transaction {transaction2} is {'valid' if valid else 'invalid'}. Reason: {reason}")
