use anchor_lang::prelude::*;

declare_id!("SonicBridge11111111111111111111111111111111");

#[program]
pub mod sonic_bridge {
    use super::*;

    /// Locks assets on a localized Sonic Hypergrid execution space to signal cross-chain bridging.
    pub fn lock_assets(ctx: Context<LockAssets>, amount: u64, target_solana_recipient: Pubkey) -> Result<()> {
        let state_account = &mut ctx.accounts.bridge_state;
        
        state_account.amount = amount;
        state_account.recipient = target_solana_recipient;
        state_account.is_settled = false;

        // In production, token balances are CPI-transferred into a secure PDA-controlled vault account
        
        msg!("SonicBridgeEvent: Assets Locked. Amount: {}, Recipient: {}", amount, target_solana_recipient);
        Ok(())
    }
}

#[account]
pub struct BridgeState {
    pub amount: u64,
    pub recipient: Pubkey,
    pub is_settled: bool,
}

#[derive(Accounts)]
pub struct LockAssets<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 1)]
    pub bridge_state: Account<'info, BridgeState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
