use std::path::Path;

use solana_client::rpc_client::RpcClient;
#[allow(unused)]
use solana_rust_transfer::{check_balance, create_keypair, request_air_drop, transfer_funds};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;

const URL: &str = "https://api.devnet.solana.com";
const TRANSFER_AMOUNT: f64 = 0.5;

static RECEIVER: Pubkey = pubkey!("GGwwvaQ9X88qAYc6ZzJcFoFdcXQf5kEZQZuu4hCwSeho");

fn main() {
    let rpc_client = RpcClient::new(URL);

    // let sender = create_keypair();
    // let receiver = create_keypair();

    // Sender keypair
    // Read keypair from a file
    let filename = &format!("{}/.config/solana/id.json", env!("HOME"));
    let path = Path::new(filename);
    let msg = format!("Read keypair file {} failed", path.to_string_lossy());
    let sender = read_keypair_file(path).expect(&msg);

    // Receiver public key from SOL addresss
    // base58 string of public key
    // let receiver = Pubkey::from_str("GGwwvaQ9X88qAYc6ZzJcFoFdcXQf5kEZQZuu4hCwSeho").unwrap();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", RECEIVER);

    let add_balance_with_airdrop = false;

    if add_balance_with_airdrop {
        let airdrop_result = request_air_drop(&rpc_client, &sender.pubkey(), 5.0);
        // if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &sender.pubkey(), 5.0) {
        if airdrop_result.is_ok() {
            let airdrop_signature = airdrop_result.unwrap();
            println!("Airdrop finished! Signature: {:?}", airdrop_signature);

            transfer(rpc_client, sender, RECEIVER, TRANSFER_AMOUNT);
        } else {
            let err = airdrop_result.unwrap_err();
            println!("Airdrop failed: {}", err);
        }
    } else {
        transfer(rpc_client, sender, RECEIVER, TRANSFER_AMOUNT);
    }
}

fn transfer(
    rpc_client: RpcClient,
    sender: solana_sdk::signature::Keypair,
    receiver: Pubkey,
    amount: f64,
) {
    if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
        println!("Sender balance: {:?}", balance);
    }

    match transfer_funds(&rpc_client, &sender, &receiver, amount) {
        Ok(sig) => {
            println!("Transfer of {:?} finished. Signature: {:?}", amount, sig);
            if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                println!("Sender balance after transfer: {:?}", balance);
            }
            if let Ok(balance) = check_balance(&rpc_client, &receiver) {
                println!("Receiver balance after transfer: {:?}", balance);
            }
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
