use candid::Principal;

#[path = "../src/lib.rs"]
mod wallet_backend;
use wallet_backend::*;

#[cfg(test)]
mod tests {
    use super::*;
    use wallet_backend::TOKENWALLET_MAP;  

    // Setup function to initialize the wallet for each test
    fn setup() {
        TOKENWALLET_MAP.with(|map| {
            let mut map = map.borrow_mut();
            map.insert(0, TokenWallet::new());
        });
    }

    // Helper to set balance in stable storage
    fn set_balance_in_storage(principal: Principal, amount: u64) {
        TOKENWALLET_MAP.with(|map| {
            let mut map = map.borrow_mut();
            let mut wallet = map.get(&0).unwrap_or_else(TokenWallet::new);
            wallet.set_balance(principal, amount);
            map.insert(0, wallet);
        });
    }

    fn mock_caller() -> Principal {
        Principal::from_text("2vxs6-xaeaa-aaaaa-qaafq-cai").unwrap()
    }

    #[test]
    fn test_initial_balance() {
        setup();
        let caller = mock_caller();
        let balance = icrc2_balance_of(caller);
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_transfer_success() {
        setup();
        let sender = mock_caller();
        let recipient = Principal::from_text("3vxs6-xaeaa-aaaaa-qaafq-cai").unwrap();

        // Set initial balance for sender in stable storage
        set_balance_in_storage(sender, 100);

        // Transfer 50 tokens
        let result = icrc2_transfer(recipient, 50);
        assert!(result.is_ok());

        // Verify balances using the public API
        assert_eq!(icrc2_balance_of(sender), 50);
        assert_eq!(icrc2_balance_of(recipient), 50);
    }

    #[test]
    fn test_transfer_insufficient_funds() {
        setup();
        let sender = mock_caller();
        let recipient = Principal::from_text("3vxs6-xaeaa-aaaaa-qaafq-cai").unwrap();

        // Set initial balance of 30 tokens
        set_balance_in_storage(sender, 30);

        // Try to transfer 50 tokens
        let result = icrc2_transfer(recipient, 50);
        assert_eq!(result, Err(TransferError::InsufficientFunds));
    }

    #[test]
    fn test_transfer_invalid_recipient() {
        setup();
        let sender = mock_caller();
        let invalid_recipient = Principal::anonymous();

        // Set initial balance
        set_balance_in_storage(sender, 100);

        // Try to transfer to invalid recipient
        let result = icrc2_transfer(invalid_recipient, 50);
        assert_eq!(result, Err(TransferError::InvalidRecipient));
    }

    #[test]
    fn test_metadata_query() {
        setup();
        let metadata = icrc2_metadata();

        assert_eq!(metadata.name, "MyToken");
        assert_eq!(metadata.symbol, "MTK");
        assert_eq!(metadata.decimals, 8);
        assert!(metadata.fee.is_none());
    }

    #[test]
    fn test_total_supply_query() {
        setup();
        let total_supply = icrc2_total_supply();
        assert_eq!(total_supply, 1_000_000_000);
    }

    #[test]
    fn test_balance_query() {
        setup();
        let caller = mock_caller();
        
        // Set balance in stable storage
        set_balance_in_storage(caller, 100);
        
        // Query balance
        let balance = icrc2_balance_of(caller);
        assert_eq!(balance, 100);
    }
}