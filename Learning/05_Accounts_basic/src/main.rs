use solana_commitment_config::CommitmentConfig;
use solana_sdk::{pubkey, signature::Keypair, signer::Signer};
use solana_sdk::pubkey::Pubkey;
use solana_client::nonblocking::rpc_client::RpcClient;
#[tokio::main]
async fn main() {
   
   // create accounts 
    let keypair = Keypair::new();
    // println!("Public key is {:?}", keypair.pubkey());
    // println!("Private key is {:?}", keypair.to_bytes());

    // PDA's
    let program_id = pubkey!("11111111111111111111111111111111");
    let seeds = [b"Shrey Vats is crazy".as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_id);

    // println!("PDA :- {:?}", pda);
    // println!("Bump :- {:?}", bump);

    fetch().await;
}

// The example below fetches the Token Program account. Notice that the executable field is set to true, indicating the account is a program.
async fn fetch (){
    let connection = RpcClient::new_with_commitment("https://api.mainnet-beta.solana.com".to_string(),CommitmentConfig::confirmed() );

    let program_id = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    let account_infos = connection.get_account(&program_id).await;

    println!("{:?}", account_infos);
}