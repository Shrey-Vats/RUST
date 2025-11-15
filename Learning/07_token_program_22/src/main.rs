use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::transaction::Transaction;
use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer};
use solana_system_interface::instruction::create_account;
use spl_token_2022_interface::state::Mint;
use spl_token_2022_interface::{
    id as token_2022_program_id, instruction::initialize_mint, 
};

#[tokio::main]
async fn main() {
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let recent_blockhash = match client.get_latest_blockhash().await {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    let fee_payer = Keypair::new();

    let airdrop_signature = match client
        .request_airdrop(&fee_payer.pubkey(), 1_000_000_000)
        .await
    {
        Ok(sig) => sig,
        Err(err) => {
            eprintln!("Airdrop failed: {err}");
            return;
        }
    };

    loop {
        let confirmed = match client.confirm_transaction(&airdrop_signature).await {
            Ok(bool) => bool,
            Err(err) => {
                eprintln!("Conformend transation error: {err}");
                return;
            }
        };

        if confirmed {
            break;
        }
    }

    let mint = Keypair::new();

    let space = Mint::LEN;
    let rent = match client.get_minimum_balance_for_rent_exemption(space).await {
        Ok(rent) => rent,
        Err(err) => {
            eprintln!("Error during rent: {err}");
            return;
        }
    };

    let create_account_instraction = create_account(
        &fee_payer.pubkey(),
        &mint.pubkey(),
        rent,
        space as u64,
        &token_2022_program_id(),
    );

    let initialize_mint_instraction = match initialize_mint(
        &token_2022_program_id(),
        &mint.pubkey(),
        &fee_payer.pubkey(),
        Some(&fee_payer.pubkey()),
        9,
    ) {
        Ok(inst) => inst,
        Err(err) => {
            eprintln!("Error in inisilising: {err}");
            return;
        }
    };

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instraction, initialize_mint_instraction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        recent_blockhash
    );

    let transaction_signature = match client.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => signature,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    println!("Mint address: {:?}", mint.pubkey());
    println!("Transaction Signature: {:?}", transaction_signature );

    let account_info = match client.get_account(&mint.pubkey()).await {
        Ok(info) => info,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    println!("Account info: {:?}", account_info);
}
