use crate::*;
use anchor_spl::token::Token;

/// Zeta Context
/// Leave this as is, it defines the instruction context for the zeta program

#[derive(Accounts, Clone)]
pub struct InitializeMarginAccount<'info> {
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct InitializeSpreadAccount<'info> {
    #[account(mut)]
    /// CHECK:
    pub spread_account: UncheckedAccount<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct Deposit<'info> {
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub socialized_loss_account: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK:
    pub state: AccountInfo<'info>,
    /// CHECK:
    pub greeks: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct Withdraw<'info> {
    /// CHECK:
    pub state: AccountInfo<'info>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub user_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub greeks: AccountInfo<'info>,
    /// CHECK:
    pub oracle: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub socialized_loss_account: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct InitializeOpenOrders<'info> {
    /// CHECK:
    pub state: AccountInfo<'info>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    /// CHECK:
    pub dex_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    /// CHECK:
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    pub market: AccountInfo<'info>,
    /// CHECK:
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub open_orders_map: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

// Market accounts are the accounts used to place orders against the dex minus
// common accounts, i.e., program ids, sysvars, and the `pc_wallet`.
#[derive(Accounts, Clone)]
pub struct MarketAccounts<'info> {
    #[account(mut)]
    /// CHECK:
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub asks: AccountInfo<'info>,
    // The `spl_token::Account` that funds will be taken from, i.e., transferred
    // from the user into the market's vault.
    //
    // For bids, this is the base currency. For asks, the quote.
    // This has to be owned by serum_authority PDA as serum checks that the owner
    // of open orders also owns this token account
    #[account(mut)]
    /// CHECK:
    pub order_payer_token_account: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    #[account(mut)]
    /// CHECK:
    pub coin_vault: AccountInfo<'info>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    #[account(mut)]
    /// CHECK:
    pub pc_vault: AccountInfo<'info>,
    // User wallets, used for settling.
    #[account(mut)]
    /// CHECK:
    pub coin_wallet: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub pc_wallet: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct PlaceOrder<'info> {
    /// CHECK:
    pub state: AccountInfo<'info>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK:
    pub dex_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK:
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub greeks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub open_orders: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub market_accounts: MarketAccounts<'info>,
    /// CHECK:
    pub oracle: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub market_node: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub market_mint: AccountInfo<'info>,
    /// CHECK:
    pub mint_authority: AccountInfo<'info>,
}

// Shared accounts required for cancel order
#[derive(Accounts, Clone)]
pub struct CancelAccounts<'info> {
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    /// CHECK:
    pub state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    /// CHECK:
    pub dex_program: AccountInfo<'info>,
    /// CHECK:
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub event_queue: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct CancelOrder<'info> {
    /// CHECK:
    pub authority: AccountInfo<'info>,
    pub cancel_accounts: CancelAccounts<'info>,
}

#[derive(Accounts, Clone)]
pub struct Liquidate<'info> {
    /// CHECK:
    pub state: AccountInfo<'info>,
    pub liquidator: Signer<'info>,
    #[account(mut)]
    /// CHECK:
    pub liquidator_margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub greeks: AccountInfo<'info>,
    /// CHECK:
    pub oracle: AccountInfo<'info>,
    /// CHECK:
    pub market: AccountInfo<'info>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub liquidated_margin_account: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct PositionMovement<'info> {
    /// CHECK:
    pub state: AccountInfo<'info>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub spread_account: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK:
    pub greeks: AccountInfo<'info>,
    /// CHECK:
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct TransferExcessSpreadBalance<'info> {
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub spread_account: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
}
