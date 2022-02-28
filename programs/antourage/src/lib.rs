use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use metaplex_token_metadata::instruction::{sign_metadata, update_metadata_accounts};
use metaplex_token_metadata::state::{Creator, Data, Metadata, PREFIX};
use metaplex_token_metadata::utils::assert_edition_valid;
use std::str::FromStr;

declare_id!("HkTeUJjPM18CGdUXBEDJusi9Wfnk2AwC5AD7PwVheQCe");

#[program]
pub mod antourage {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let new_counter = &mut ctx.accounts.counter;
        new_counter.current_count = 0;
        new_counter.limit = 5000;
        new_counter.authority = ctx.accounts.admin.key();
        new_counter.base_uri = String::from("https://testantourage.s3.eu-north-1.amazonaws.com");
        new_counter.creator =
            Pubkey::from_str("FKviGeU7L1djHrbf21z2ijK2N9KAREagtTjhoc9KQZn6").unwrap();

        Ok(())
    }
    pub fn update_counter_authority(
        ctx: Context<UpdateCounterAuthority>,
        bump: u8,
        new_base_uri: Option<String>,
        new_creator: Option<Pubkey>,
    ) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.new_auth.key();
        if let Some(base_uri) = new_base_uri {
            counter.base_uri = base_uri;
        }
        if let Some(creator) = new_creator {
            counter.creator = creator;
        }

        Ok(())
    }
    pub fn update_counter_limit(
        ctx: Context<UpdateCounterLimit>,
        bump: u8,
        new_limit: u64,
    ) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.limit = new_limit;

        Ok(())
    }
    pub fn buy_ball(ctx: Context<BuyBall>, counter_bump: u8, program_signer_bump:u8) -> ProgramResult {
        //validate red lion
        //validate golf ball
        //increment index on counter
        //sign as creator

        validate_red_lion(
            &ctx.accounts.user,
            &ctx.accounts.red_lion_mint_account,
            &ctx.accounts.red_lion_token_account,
            &ctx.accounts.red_lion_metadata_account,
            &ctx.accounts.token_metadata_program,
        )?;

        validate_golf_ball(
            &ctx.accounts.user,
            &ctx.accounts.golf_mint_account,
            &ctx.accounts.golf_token_account,
            &ctx.accounts.golf_metadata_account,
            &ctx.accounts.golf_master_edition,
            &ctx.accounts.program_pda_signer,
            &ctx.accounts.token_metadata_program,
            &ctx.accounts.counter,
        )?;

        let counter_account = &mut ctx.accounts.counter;
        counter_account.current_count+=1;

        sign_golf(&ctx, &program_signer_bump)?;



        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, address = Pubkey::from_str("3ANuEmA1Prg6STW7LUGCvc5NYZRbmGVhAvGGT7gPUUVg").unwrap())]
    pub admin: Signer<'info>,

    #[account(
    init,
    payer = admin,
    seeds = [b"counter"],
    bump,
    space = 240
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(counter_bump:u8)]
pub struct UpdateCounterAuthority<'info> {
    pub authority: Signer<'info>,

    pub new_auth: SystemAccount<'info>,

    #[account(
    mut,
    has_one = authority,
    seeds = [b"counter"],
    bump = counter_bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
#[instruction(counter_bump:u8)]
pub struct UpdateCounterLimit<'info> {
    pub authority: Signer<'info>,

    #[account(
    mut,
    has_one = authority,
    seeds = [b"counter"],
    bump = counter_bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
#[instruction(counter_bump:u8, program_signer_bump:u8)]
pub struct BuyBall<'info> {
    //creator for metadata acc
    #[account(seeds = [b"creator"], bump = program_signer_bump)]
    pub program_pda_signer: AccountInfo<'info>,

    pub user: Signer<'info>,

    // //needs to have a valid red lion NFT
    #[account(mut)]
    red_lion_token_account: Box<Account<'info, TokenAccount>>,
    red_lion_mint_account: Box<Account<'info, Mint>>,
    #[account(mut)]
    red_lion_metadata_account: UncheckedAccount<'info>,

    //about to be created golf ball
    #[account(mut)]
    golf_token_account: Box<Account<'info, TokenAccount>>,
    golf_mint_account: Box<Account<'info, Mint>>,
    golf_master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    golf_metadata_account: AccountInfo<'info>,

    //counter to track number created
    #[account(
    mut,
    seeds = [b"counter"],
    bump = counter_bump,
    )]
    pub counter: Account<'info, Counter>,

    pub token_program: Program<'info, Token>,

    #[account(address = metaplex_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub limit: u64,
    pub current_count: u64,
    pub authority: Pubkey,
    pub base_uri: String,
    pub creator: Pubkey,
}

pub fn validate_golf_ball<'info>(
    user: &Signer<'info>,
    golf_mint: &Account<'info,Mint>,
    golf_tokenacc: &Account<'info,TokenAccount>,
    golf_metadataacc: &AccountInfo<'info>,
    golf_master_edition: &AccountInfo<'info>,
    program_pda_signer: &AccountInfo<'info>,
    token_metadata_program: &AccountInfo<'info>,
    counter_account: &Account<'info,Counter>, 
) -> Result<()> {
    //validate token account
    assert_eq!(golf_tokenacc.owner, user.key());
    assert_eq!(golf_tokenacc.amount, 1);
    assert_eq!(golf_tokenacc.mint, golf_mint.key());

    //Check seeds of metadata account
    let metadata_seed = &[
        PREFIX.as_bytes(),
        token_metadata_program.key.as_ref(),
        golf_tokenacc.mint.as_ref(),
    ];
    let (metadata_derived_key, _bump_seed) =
        Pubkey::find_program_address(metadata_seed, token_metadata_program.key);

    assert_eq!(metadata_derived_key, golf_metadataacc.key());

    //Check if initialized
    if golf_metadataacc.data_is_empty() {
        return Err(ErrorCode::NotInitialized.into());
    };

    //check master edition
    assert_edition_valid(
        token_metadata_program.key,
        &golf_mint.key(),
        &golf_master_edition,
    )
    .unwrap();

    if golf_master_edition.data_is_empty() {
        return Err(ErrorCode::NotInitialized.into());
    };

    //get metadata account struct
    let metadata_full_account = &mut Metadata::from_account_info(&golf_metadataacc)?;

    //Check creators array
    let program_signer_creator = Creator {
        address: counter_account.creator,
        share: 100,
        verified: false,
    };
    let full_share_creator = Creator {
        address: program_pda_signer.key(),
        share: 0,
        verified: false,
    };

    let expected_creators_array = vec![program_signer_creator, full_share_creator];


    //Check isMutable
    assert_eq!(metadata_full_account.is_mutable, true);

    //Check base URI
    let mut created_uri =  counter_account.base_uri.clone();

    //add on current count
    let add_on =
        "/".to_string() + &counter_account.current_count.to_string() + &".json".to_string();
    created_uri.push_str(&add_on);


    let new_data = Data {
        name: "Antourage tester".to_string(),
        symbol: "ANT".to_string(),
        uri: created_uri,
        seller_fee_basis_points: 720,
        creators: Some(expected_creators_array),
    };

    let metadata_infos = [
        golf_metadataacc.to_account_info(),
        program_pda_signer.to_account_info(),
        user.to_account_info()
    ];

    anchor_lang::solana_program::program::invoke(
        &update_metadata_accounts(
            token_metadata_program.key(),
            golf_metadataacc.key(),
            user.key(),
            Some(Pubkey::from_str("FKviGeU7L1djHrbf21z2ijK2N9KAREagtTjhoc9KQZn6").unwrap()),
            Some(new_data),
            None,
        ),
        &metadata_infos,
    )?;
    

    Ok(())
}

pub fn validate_red_lion(
    user: &Signer,
    red_mint: &Account<Mint>,
    red_tokenacc: &Account<TokenAccount>,
    red_metadataacc: &AccountInfo,
    token_metadata_program: &AccountInfo,
) -> Result<()> {
    assert_eq!(red_tokenacc.owner, user.key());
    assert_eq!(red_tokenacc.amount, 1);
    assert_eq!(red_tokenacc.mint, red_mint.key());

    //Check seeds of metadata account
    let metadata_seed = &[
        PREFIX.as_bytes(),
        token_metadata_program.key.as_ref(),
        red_tokenacc.mint.as_ref(),
    ];
    let (metadata_derived_key, _bump_seed) =
        Pubkey::find_program_address(metadata_seed, token_metadata_program.key);

    assert_eq!(metadata_derived_key, red_metadataacc.key());

    //Check if initialized
    if red_metadataacc.data_is_empty() {
        return Err(ErrorCode::NotInitialized.into());
    };
    let metadata_full_account = &mut Metadata::from_account_info(&red_metadataacc)?;

    //Check creators array
    let program_signer_creator = Creator {
        address: Pubkey::from_str("12DyUGEZNzsUK1FqX72nmadH8KnHsKcRZdoBRerbjVRx").unwrap(),
        share: 0,
        verified: false,
    };
    let full_share_creator = Creator {
        address: Pubkey::from_str("3ANuEmA1Prg6STW7LUGCvc5NYZRbmGVhAvGGT7gPUUVg").unwrap(),
        share: 100,
        verified: true,
    };

    let expected_creators_array = vec![program_signer_creator, full_share_creator];

    let current_creators = metadata_full_account.data.creators.as_ref().unwrap();
    //DISABLED FOR NOW
    if current_creators != &expected_creators_array {
        return Err(ErrorCode::WrongCreators.into());
    }

    Ok(())
}

fn sign_golf(
    ctx: &Context<BuyBall>,
    program_signer_bump: &u8,
    //token_metadata, metadatacc, programpdasigner
) -> Result<()> {
    //sign
    let program_signer_seeds = ["creator".as_bytes(), &[*program_signer_bump]];

    let metadata_infos = [
        ctx.accounts.golf_metadata_account.clone(),
        ctx.accounts.program_pda_signer.to_account_info(),
    ];

    anchor_lang::solana_program::program::invoke_signed(
        &sign_metadata(
            *ctx.accounts.token_metadata_program.key,
            *ctx.accounts.golf_metadata_account.key,
            *ctx.accounts.program_pda_signer.key,
        ),
        &metadata_infos,
        &[&program_signer_seeds],
    )?;

    Ok(())
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid aurahma pack chosen")]
    InvalidPack,

    #[msg("Out of Pack 1 aurahmas")]
    OutOfL1,

    #[msg("Out of Pack 2 aurahmas")]
    OutOfL2,

    #[msg("Out of Pack 3 aurahmas")]
    OutOfL3,

    #[msg("Metadata account not Initialized")]
    NotInitialized,

    #[msg("Wrong creators or already signed for program signer")]
    WrongCreators,

    #[msg("Wrong owners")]
    WrongOwner,

    #[msg("Insufficient funds in a token account")]
    InsufficientFunds,

    #[msg("Something went wrong with token accounts")]
    GenericError,
}
