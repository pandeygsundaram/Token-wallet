# Token Wallet on ICP Blockchain

## Overview
This project is a secure and feature-rich token wallet implemented in Rust for the Internet Computer Protocol (ICP) blockchain. The wallet supports essential functionalities such as sending and receiving ICRC2 tokens, querying balances, and fetching metadata.

## Features
- **Send Tokens**: Transfer tokens from one user to another securely.
- **Receive Tokens**: Update wallet balance upon receiving tokens.
- **Balance Display**: Query and display the token balance for a specific user.
- **Total Supply Query**: Fetch the total token supply.
- **Error Handling**: Robust handling for insufficient funds, invalid recipients, and other edge cases.

## Setup Instructions

### 1. Prerequisites
- Rust programming language installed.
- DFX SDK (for deploying on the Internet Computer) installed.

### 2. Clone the Repository
```bash
git clone https://github.com/pandeygsundaram/Token-wallet
cd Token-wallet
```

### 3. Build the Project
Compile the Rust code:
```bash
cargo build
```

### 4. Deploy to Local ICP Network
Start the local ICP network and deploy the canister:
```bash
dfx start --background
dfx deploy
```

### 5. Interact with the Smart Contract
Once deployed, you can interact with the token wallet through the following queries and updates:

#### Query Token Metadata
Fetch metadata such as name, symbol, decimals, and fee:
```rust
let metadata = icrc2_metadata();
```

#### Get Balance
Query the balance of a specific user:
```rust
let balance = icrc2_balance_of(principal);
```

#### Send Tokens
Transfer tokens from one user to another:
```rust
let result = icrc2_transfer(to, amount);
```

#### Total Supply
Fetch the total supply of the token:
```rust
let total_supply = icrc2_total_supply();
```

### 6. Running Tests
You can run the unit tests to validate the functionality of the wallet:
```bash
cargo test
```

## Functionality

### Token Wallet
The `TokenWallet` struct is the core of the wallet, storing token balances and metadata. Key fields include:
- **balances**: A `HashMap` storing user balances identified by their Principal.
- **total_supply**: The total supply of tokens (e.g., 1,000,000,000 tokens).
- **name**: The token name (e.g., "MyToken").
- **symbol**: The token symbol (e.g., "MTK").
- **decimals**: Number of decimals (e.g., 8).

### Functions
- **`icrc2_metadata`**: Retrieves token metadata.
- **`icrc2_balance_of`**: Returns the balance of a specific user.
- **`icrc2_transfer`**: Transfers tokens between users.
- **`icrc2_total_supply`**: Returns the total token supply.

### Error Handling
- **InsufficientFunds**: Raised when the sender has insufficient tokens.
- **InvalidRecipient**: Raised when the recipient’s address is invalid.

### Example Usage
#### Create a New Token Wallet
```rust
let token_wallet = TokenWallet::new();
```

#### Query User Balance
```rust
let balance = token_wallet.get_balance(principal);
```

#### Transfer Tokens
```rust
let result = icrc2_transfer(recipient, 50);
```

#### Fetch Metadata
```rust
let metadata = icrc2_metadata();
```

## Testing
The project includes unit tests to validate:
- **Initial Balance**: Verifies a new wallet has an initial balance of 0 tokens.
- **Transfer Success**: Checks that transfers update balances correctly.
- **Transfer Failure**: Validates error handling for insufficient funds and invalid recipients.
- **Metadata Query**: Ensures correct fetching of metadata.
- **Total Supply Query**: Verifies the total token supply.

To run the tests:
```bash
cargo test
```

## License

This project is licensed under the MIT License. See the full license text below:

```
MIT License

Copyright (c) 2025 ICP Token Wallet

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

