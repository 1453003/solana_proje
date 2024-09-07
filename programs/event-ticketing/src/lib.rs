use anchor_lang::prelude::*;

declare_id!("8SMtKx33R6woBov327Dw3kpduKaCUciNNVCMjcxkYWHa");

#[program]
pub mod event_ticketing {
    use super::*;

    pub fn create_event(ctx: Context<CreateEvent>, name: String, date: i64) -> Result<()> {
        let event = &mut ctx.accounts.event_account;
        event.name = name;
        event.date = date;
        event.organizer = ctx.accounts.signer.key();
        msg!("Event created: {}!", event.name);
        Ok(())
    }

    pub fn mint_ticket(ctx: Context<MintTicket>, ticket_id: u64, price: u64) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket_account;
        ticket.ticket_id = ticket_id;
        ticket.price = price;
        ticket.owner = ctx.accounts.signer.key();
        msg!("Ticket minted: {} with price {}!", ticket_id, price);
        Ok(())
    }

    pub fn transfer_ticket(ctx: Context<TransferTicket>, new_owner: Pubkey) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket_account;

        // Alıcı, biletin fiyatını ödemeli
        let price = ticket.price;
        **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? -= price;
        **ctx.accounts.ticket_owner.to_account_info().try_borrow_mut_lamports()? += price;

        ticket.owner = new_owner;
        msg!("Ticket transferred to new owner!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = signer, space = 64)]
    pub event_account: Account<'info, Event>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTicket<'info> {
    #[account(init, payer = signer, space = 64)]
    pub ticket_account: Account<'info, Ticket>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTicket<'info> {
    #[account(mut)]
    pub ticket_account: Account<'info, Ticket>,

    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// CHECK: This is the current owner of the ticket who will receive the payment
    pub ticket_owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Event {
    pub name: String,
    pub date: i64,
    pub organizer: Pubkey,
}

#[account]
pub struct Ticket {
    pub ticket_id: u64,
    pub price: u64,
    pub owner: Pubkey,
}
