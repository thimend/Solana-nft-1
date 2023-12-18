use anchor_lang::prelude::*;

declare_id!("3uCGttvR5yP62bjWbMifHPsj64Yq5UkveAhvsghpi8vt");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;

    Ok(())
  }
  
	// Outra função uhul!
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
    // Obtem a referencia para a conta e incrementa total_gifs.
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    	// Constroi o struct.
      let item = ItemStruct {
        gif_link: gif_link.to_string(),
        user_address: *user.to_account_info().key,
        gif_votes: 0,
      };

	  // Adiciona ele ao vetor gif_list.
    base_account.gif_list.push(item);

    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn vote_gif(ctx: Context<VoteGif>, index_gif: u16) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let index: usize = index_gif as usize;
    base_account.gif_list[index].gif_votes += 1;
    Ok(())
  }
}



#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Especifica que dados queremos no Contexto AddGif
// Obtendo um controle sobre o fluxo das coisas :)?
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VoteGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

// Crie uma estrutura personalizada para trabalharmos.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub gif_votes: u128,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
