use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Re-exporting these modules to be used in the main program
pub use instruction::*;
pub use processor::*;
pub use verify_proof::*;

pub mod instruction;
pub mod processor;
pub mod verify_proof;
pub mod account_info;


#[program]
pub mod confidential_transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct ConfidentialTransferMint {
    /// Authority to modify the `ConfidentialTransferMint` configuration and to
    /// approve new accounts (if `auto_approve_new_accounts` is true)
    pub authority: Pubkey,

    /// Indicate if newly configured accounts must be approved by the
    /// `authority` before they may be used by the user.
    pub auto_approve_new_accounts: bool,

    /// Authority to decode any transfer amount in a confidential transfer.
    pub auditor_elgamal_pubkey: Pubkey, // Assuming this is a pubkey for simplicity
}

#[account]
#[derive(Default)]
pub struct ConfidentialTransferAccount {
    /// `true` if this account has been approved for use.
    pub approved: bool,

    /// The public key associated with ElGamal encryption
    pub elgamal_pubkey: Pubkey, // Assuming this is a pubkey

    /// The low 16 bits of the pending balance (encrypted)
    pub pending_balance_lo: Vec<u8>, // Using Vec<u8> for encrypted data

    /// The high 48 bits of the pending balance (encrypted)
    pub pending_balance_hi: Vec<u8>,

    /// The available balance (encrypted)
    pub available_balance: Vec<u8>,

    /// The decryptable available balance
    pub decryptable_available_balance: Vec<u8>,

    /// If `false`, the extended account rejects any incoming confidential transfers
    pub allow_confidential_credits: bool,

    /// If `false`, the base account rejects any incoming transfers
    pub allow_non_confidential_credits: bool,

    /// The total number of `Deposit` and `Transfer` instructions that have
    /// credited `pending_balance`
    pub pending_balance_credit_counter: u64,

    /// The maximum number of `Deposit` and `Transfer` instructions that can
    /// credit `pending_balance` before the `ApplyPendingBalance`
    /// instruction is executed
    pub maximum_pending_balance_credit_counter: u64,

    /// The `expected_pending_balance_credit_counter` value that was included in
    /// the last `ApplyPendingBalance` instruction
    pub expected_pending_balance_credit_counter: u64,

    /// The actual `pending_balance_credit_counter` when the last
    /// `ApplyPendingBalance` instruction was executed
    pub actual_pending_balance_credit_counter: u64,
}

impl ConfidentialTransferAccount {
    /// Check if a `ConfidentialTransferAccount` has been approved for use.
    pub fn approved(&self) -> Result<()> {
        if self.approved {
            Ok(())
        } else {
            err!(ErrorCode::ConfidentialTransferAccountNotApproved)
        }
    }

    /// Check if a `ConfidentialTransferAccount` is in a closable state.
    pub fn closable(&self) -> Result<()> {
        if self.pending_balance_lo.is_empty()
            && self.pending_balance_hi.is_empty()
            && self.available_balance.is_empty()
        {
            Ok(())
        } else {
            err!(ErrorCode::ConfidentialTransferAccountHasBalance)
        }
    }
}


#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum ErrorCode {
    #[msg("Confidential transfer account not approved.")]
    ConfidentialTransferAccountNotApproved,
    #[msg("Confidential transfer account has balance.")]
    ConfidentialTransferAccountHasBalance,
} 