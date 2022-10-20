use crate::zeta_context::*;
use crate::*;

// CPI Program Context
// Edit this as you wish for your own program instructions

#[derive(Accounts)]
pub struct InitializeMarginAccountCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub initialize_margin_cpi_accounts: InitializeMarginAccount<'info>,
}

#[derive(Accounts)]
pub struct InitializeSpreadAccountCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub initialize_spread_cpi_accounts: InitializeSpreadAccount<'info>,
}

#[derive(Accounts)]
pub struct DepositCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub deposit_cpi_accounts: Deposit<'info>,
}

#[derive(Accounts)]
pub struct WithdrawCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub withdraw_cpi_accounts: Withdraw<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub initialize_open_orders_cpi_accounts: InitializeOpenOrders<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrderCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub place_order_cpi_accounts: PlaceOrder<'info>,
}

#[derive(Accounts)]
pub struct CancelOrderCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub cancel_order_cpi_accounts: CancelOrder<'info>,
}

#[derive(Accounts)]
pub struct PositionMovementCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub position_movement_cpi_accounts: PositionMovement<'info>,
}

#[derive(Accounts)]
pub struct TransferExcessSpreadBalanceCaller<'info> {
    /// CHECK:
    pub zeta_program: AccountInfo<'info>,
    pub transfer_excess_spread_balance_cpi_accounts: TransferExcessSpreadBalance<'info>,
}

#[derive(Accounts)]
pub struct ReadProgramData<'info> {
    /// CHECK:
    pub state: AccountInfo<'info>,
    /// CHECK:
    pub zeta_group: AccountInfo<'info>,
    /// CHECK:
    pub margin_account: AccountInfo<'info>,
    /// CHECK:
    pub greeks: AccountInfo<'info>,
    /// CHECK:
    pub oracle: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PositionMovementArg {
    pub index: u8,
    pub size: i64,
}
