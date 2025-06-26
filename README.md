# Confidential Balance Anchor Program

This project is an Anchor-based Solana smart contract that implements the **Confidential Transfer Extension** for SPL Tokens. It enables private, secure token transfers where balances and transaction amounts are kept confidential using zero-knowledge proofs (ZKPs) and ElGamal encryption.

This repository contains the ongoing conversion of a raw Solana program into a more robust and developer-friendly Anchor program.


## Contacts

-[telegram](https://t.me/caterpillardev)
-[twitter](https://x.com/caterpillardev)

## Key Features

- **Confidential Transfers**: Send and receive tokens without revealing amounts or balances on-chain.
- **Encrypted Balances**: All account balances are stored in an encrypted format.
- **Zero-Knowledge Proofs**: Cryptographic proofs are used to validate transactions without exposing sensitive data.
- **Anchor Framework**: Built with Anchor for a more secure, ergonomic, and maintainable codebase.
- **Fine-Grained Controls**: Supports features like enabling/disabling confidential credits, applying pending balances, and handling confidential fees.

## Program Architecture

The program is structured as a standard Anchor project.

### Core Components

- **`lib.rs`**: The main program entry point, containing instruction handlers and account definitions.
- **`state/` (future)**: Will contain the definitions for on-chain accounts.
- **`instructions/` (future)**: Will define the context for each instruction handler.

### Key Accounts

- **`ConfidentialTransferMint`**: An account that configures confidential transfer settings for a specific token mint. It holds keys for auditing and parameters for account approval.
- **`ConfidentialTransferAccount`**: An account that holds a user's confidential balance, including encrypted available and pending balances, and configuration for allowing transfers.

## How It Works

The program leverages advanced cryptography to ensure privacy and correctness:

1.  **Initialization**: A token mint is configured for confidential transfers by creating a `ConfidentialTransferMint` account.
2.  **Account Configuration**: Users configure their token accounts for confidential transfers, creating a `ConfidentialTransferAccount`.
3.  **Deposits**: Users can deposit tokens into their confidential account. The amount is added to an encrypted "pending balance."
4.  **Applying Balances**: A user can trigger an instruction to roll their pending balance into their main available balance, all while keeping the amounts encrypted.
5.  **Transfers**: Users can transfer tokens confidentially by providing a zero-knowledge proof that validates the transaction without revealing the amount. The program verifies this proof and updates the encrypted balances of the sender and receiver.

## Development Status

This project is currently being converted from a raw Solana program to the Anchor framework.

**Completed Steps:**
- [x] Initialized Anchor project structure (`Anchor.toml`, `programs/` directory).
- [x] Converted core state structs (`ConfidentialTransferMint`, `ConfidentialTransferAccount`) to Anchor `#[account]`s.
- [x] Defined initial custom errors using `#[error_code]`.

**Next Steps:**
- Convert each instruction from the original program (`deposit`, `withdraw`, `transfer`, etc.) into an Anchor instruction handler.
- Define `#[derive(Accounts)]` structs for each instruction to ensure type-safe account validation.
- Implement comprehensive tests using the Anchor testing framework.
- Refactor the cryptographic helpers into clean, reusable Rust modules.

## How to Build and Test

Once the conversion is more complete, you will be able to build and test the program using standard Anchor CLI commands.

```bash
# Build the program
anchor build

# Run tests
anchor test
```

Stay tuned for further development!
