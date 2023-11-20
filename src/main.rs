use dotenv::dotenv;
use std::env::var;
use std::str::FromStr;
use std::sync::Arc;

use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use ethers::types::Address;
use std::time::{Duration, SystemTime};
use tokio::task::JoinHandle;
use tx_gen::abi::token_abi::Token;

#[tokio::main]
async fn main() {
    run().await;
    println!("tx complete");
}

async fn run() {
    dotenv().ok();
    let token_address = var("TOKEN_ADDRESS").expect("Cannot detect TOKEN_ADDRESS env var");
    let private_key = var("PRIVATE_KEY").expect("Cannot detect TOKEN_ADDRESS env var");
    let eth_rpc = var("ETH_RPC").expect("Cannot detect TOKEN_ADDRESS env var");

    let l2_provider: Provider<Http> = Provider::<Http>::try_from(eth_rpc.as_str()).unwrap();
    let chain_id = l2_provider.get_chainid().await.unwrap().as_u64();
    //1212121212121212121212121212121212121212121212121212121212121212
    //3e4bde571b86929bf08e2aaad9a6a1882664cd5e65b96fff7d03e1c4e6dfa15c
    let l2_signer = Arc::new(SignerMiddleware::new(
        l2_provider.clone(),
        Wallet::from_str(private_key.as_str())
            .unwrap()
            .with_chain_id(chain_id),
    ));

    let token: Token<SignerMiddleware<Provider<Http>, _>> = Token::new(
        Address::from_str(token_address.as_str()).unwrap(),
        l2_signer.clone(),
    );

    let count = l2_provider
        .get_transaction_count(l2_signer.address(), None)
        .await;
    println!("tx count: {:?}", count.unwrap());

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let wallet1 = Wallet::new(&mut rng).with_chain_id(chain_id);
    let tx1 = token.transfer(wallet1.address(), U256::from(100000)).legacy();
    let rt: Result<_, _> = tx1.send().await;
    std::thread::sleep(Duration::from_secs(10));
    let balance = token.balance_of(wallet1.address()).await.unwrap();
    println!("balance: {:?}", balance);
    assert!(balance == U256::from(100000), "Balance is not as expected");
    println!(
        "==========>block_number_end:{:?}",
        l2_provider.get_block_number().await.unwrap()
    );
    println!("current time: {:?}", SystemTime::now());

    std::thread::sleep(Duration::from_secs(20));
}
