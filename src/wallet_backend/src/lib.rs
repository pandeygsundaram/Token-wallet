

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use ic_cdk::api;

// Define the type for memory management
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Maximum size of a token wallet value stored in memory
const MAX_VALUE_SIZE: u32 = 100;

// Define the TokenWallet struct that holds token balances and metadata
#[derive(CandidType, Deserialize)]
pub struct TokenWallet {
    // HashMap to store the balance of each user (identified by Principal)
    balances: std::collections::HashMap<Principal, u64>,
    // The total supply of the token
    total_supply: u64,
    // Name of the token
    name: String,
    // Symbol of the token
    symbol: String,
    // Decimals for the token (how many decimal places can be used)
    decimals: u8,
}

// Implement the Storable trait to allow TokenWallet to be serialized and deserialized
impl Storable for TokenWallet {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap()) // Serialize the TokenWallet struct to bytes
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes.as_ref(), Self).unwrap() // Deserialize the bytes into a TokenWallet struct
    }
}

// Implement BoundedStorable to define size limitations for the TokenWallet in memory
impl BoundedStorable for TokenWallet {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; // Max size allowed for the token wallet in stable memory
    const IS_FIXED_SIZE: bool = false;    // This wallet size is not fixed
}

// Use thread-local storage for memory management and token wallet map
thread_local! {
    // Memory manager to handle memory allocations
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // A stable BTreeMap to store the TokenWallet
    pub static TOKENWALLET_MAP: RefCell<StableBTreeMap<u64, TokenWallet, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    ));
}

// Implement the TokenWallet struct with methods to get and set balances
impl TokenWallet {
    // Create a new TokenWallet with default values
    pub fn new() -> Self {
        TokenWallet {
            balances: std::collections::HashMap::new(),
            total_supply: 1_000_000_000, // Example total supply
            name: "MyToken".to_string(),
            symbol: "MTK".to_string(),
            decimals: 8,
        }
    }

    // Get the balance for a specific Principal (user)
    pub fn get_balance(&self, address: Principal) -> u64 {
        *self.balances.get(&address).unwrap_or(&0) // If no balance exists, return 0
    }

    // Set or update the balance for a specific Principal (user)
    pub fn set_balance(&mut self, address: Principal, balance: u64) {
        self.balances.insert(address, balance); // Insert or update the balance in the HashMap
    }
}

// Metadata struct to hold token metadata (name, symbol, decimals, and optional fee)
#[derive(CandidType, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub fee: Option<u64>,  // Optional transaction fee for token transfers
}

// Enum to define possible transfer errors
#[derive(CandidType, Deserialize, Debug,PartialEq)]
pub enum TransferError {
    InsufficientFunds,   // When the sender does not have enough balance to transfer
    InvalidRecipient,    // When the recipient is invalid (e.g., zero address or uninitialized)
    Other(String),       // For any other error, a custom error message can be provided
}

// Type alias for the transfer result (either Ok or an error)
pub type TransferResult = Result<(), TransferError>;

// Query method to retrieve token metadata (name, symbol, decimals, and fee)
#[ic_cdk::query]
pub fn icrc2_metadata() -> Metadata {
    // Retrieve the token wallet from stable storage, or initialize a new one if not found
    let token_wallet = TOKENWALLET_MAP.with(|map| {
        let map = map.borrow();
        map.get(&0).unwrap_or_else(|| TokenWallet::new()) // Default TokenWallet if none exists
    });

    // Return the token metadata
    Metadata {
        name: token_wallet.name,
        symbol: token_wallet.symbol,
        decimals: token_wallet.decimals,
        fee: None, // Optional fee (not implemented here)
    }
}

// Query method to retrieve the balance of a specific owner (Principal)
#[ic_cdk::query]
pub fn icrc2_balance_of(owner: Principal) -> u64 {
    // Retrieve the token wallet from stable storage
    let token_wallet = TOKENWALLET_MAP.with(|map| {
        let map = map.borrow();
        map.get(&0).unwrap_or_else(|| TokenWallet::new()) // Default TokenWallet if none exists
    });
    
    // Return the balance for the specified owner
    token_wallet.get_balance(owner)
}

// Update method to transfer tokens from the caller to a recipient
#[ic_cdk::update]
pub fn icrc2_transfer(to: Principal, amount: u64) -> TransferResult {
    let caller = api::caller(); // Get the caller's Principal (who is initiating the transfer)
    
    // Retrieve the token wallet from stable storage, or initialize a new one if not found
    let mut token_wallet = TOKENWALLET_MAP.with(|map| {
        let map = map.borrow_mut();
        map.get(&0).unwrap_or_else(|| TokenWallet::new()) // Default TokenWallet if none exists
    });

    // Check if the caller has enough balance to complete the transfer
    let from_balance = token_wallet.get_balance(caller);
    if from_balance < amount {
        return Err(TransferError::InsufficientFunds); // Return an error if not enough funds
    }

    // Validate the recipient Principal (non-zero)
    if to == Principal::anonymous() {
        return Err(TransferError::InvalidRecipient); // Return an error if the recipient is invalid
    }

    // Update the balances: subtract from sender, add to recipient
    token_wallet.set_balance(caller, from_balance - amount);
    let to_balance = token_wallet.get_balance(to);
    token_wallet.set_balance(to, to_balance + amount);

    // Persist the updated wallet state to stable storage
    TOKENWALLET_MAP.with(|map| {
        let mut map = map.borrow_mut();
        map.insert(0, token_wallet);  // Store the updated wallet
    });

    Ok(()) // Return success
}

// Query method to retrieve the total supply of the token
#[ic_cdk::query]
pub fn icrc2_total_supply() -> u64 {
    // Retrieve the token wallet from stable storage
    let token_wallet = TOKENWALLET_MAP.with(|map| {
        let map = map.borrow();
        map.get(&0).unwrap_or_else(|| TokenWallet::new()) // Default TokenWallet if none exists
    });
    
    // Return the total supply of the token
    token_wallet.total_supply
}

ic_cdk::export_candid!();
