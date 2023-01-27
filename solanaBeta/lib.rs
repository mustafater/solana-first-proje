use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("6MhT8RYRYk13xPAc8e9jjTbHokTSwaZpXhp3ojToYZEC");

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account;
        new_account.user = ctx.accounts.user.key();
        new_account.account_id = 0;
        new_account.accunt_state = 1;

        Ok(())
    }


    pub fn item_add(
        ctx:Context<ItemAdd>,
        _id:u8,
        property:String,
        price:String,
        image:String,
        
    )-> Result<()>{
       let item_account=&mut ctx.accounts.item_account;
       let new_account=&mut ctx.accounts.new_account;
       item_account.user=ctx.accounts.user.key();
       item_account.id=new_account.account_id;
       item_account.property=property;
       item_account.price=price;
       item_account.image=image;

       new_account.account_id=new_account.account_id.checked_add(1).unwrap();

        Ok(())
    }
    pub fn item_update(
            ctx:Context<ItemUpdate>,
        id:u8,
        property:String,
        price:String,
        image:String,
    )-> Result<()>{
        let item_account=&mut ctx.accounts.item_account;
       
       item_account.id= id;
       item_account.property=property;
       item_account.price=price;
       item_account.image=image;

        Ok(())
    }

    pub fn  item_delete(
     _ctx:Context<ItemDelete>  
    )-> Result<()>{

    
        Ok(())
    }

    
}

#[account]
#[derive(Default)]
pub struct NewAccount {
    pub user: Pubkey,
    pub account_id: u8,
    pub accunt_state: u8,
    
}

#[account]
#[derive(Default)]
pub struct Item {
    pub user:Pubkey,
    pub id:u8,
    pub property:String,
    pub price:String,
    pub image:String,
    
}


#[derive(Accounts)]
pub struct InitializeUser<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)

    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(init, 
      seeds=[b"user-stats",user.key().as_ref()],
      payer = user, 
      space = 8+ 16,
      bump,
      )]
    pub new_account: Box<Account<'info, NewAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct ItemAdd<'info> {
    #[account(
        mut,
        seeds = [b"user-stats", user.key().as_ref()],
        bump,
        has_one = user,
    )]
     pub new_account: Box<Account<'info, NewAccount>>,

    #[account(
        init,
        seeds = [b"item-stas", user.key().as_ref(), &[new_account.account_id]],
        bump,
        payer = user,
        space = 2956 + 16,
    )]
    pub item_account: Box<Account<'info, Item>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
  
#[derive(Accounts)]
#[instruction(id: u8)]
pub struct ItemUpdate<'info>{
 #[account(
        mut,
        seeds = [b"user-stats", user.key().as_ref()],
        bump,
        has_one = user,
    )]
     pub new_account: Box<Account<'info, NewAccount>>,

    #[account(
        mut,
        seeds = [b"item-stas", user.key().as_ref(), &[new_account.account_id]],
        bump,
        has_one = user,
    )]
    pub item_account: Box<Account<'info, Item>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ItemDelete<'info>{
     #[account(
        mut,
        seeds = [b"user-stats", user.key().as_ref()],
        bump,
        has_one = user,
    )]
     pub new_account: Box<Account<'info, NewAccount>>,

    #[account(
        mut,
        close=user,
        seeds = [b"item-stas", user.key().as_ref(), &[new_account.account_id]],
        bump,
        has_one = user,
       
    )]
    pub item_account: Box<Account<'info, Item>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

    

    




