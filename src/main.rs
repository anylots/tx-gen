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
    let l2_provider: Provider<Http> = Provider::<Http>::try_from("http://127.0.0.1:8545").unwrap();
    let chain_id = l2_provider.get_chainid().await.unwrap().as_u64();
    //1212121212121212121212121212121212121212121212121212121212121212
    //3e4bde571b86929bf08e2aaad9a6a1882664cd5e65b96fff7d03e1c4e6dfa15c
    let l2_signer = Arc::new(SignerMiddleware::new(
        l2_provider.clone(),
        Wallet::from_str("1212121212121212121212121212121212121212121212121212121212121212")
            .unwrap()
            .with_chain_id(chain_id),
    ));

    let token: Token<SignerMiddleware<Provider<Http>, _>> = Token::new(
        Address::from_str("0x21cdbD4361A5944E4bE5B08723ecC5e2e38A9841").unwrap(),
        l2_signer.clone(),
    );

    let mut token_vec = Vec::<Token<SignerMiddleware<Provider<Http>, _>>>::new();
    let mut i = 0;
    while i < 20 {
        i += 1;
        let mut rng = rand::thread_rng();
        let wallet = Wallet::new(&mut rng).with_chain_id(chain_id);
        let tx = TransactionRequest::new()
            .to(wallet.address())
            .value(1 * 10u64.pow(18));
        l2_signer.send_transaction(tx, None).await.unwrap();
        std::thread::sleep(Duration::from_secs(5));

        //Prepare balance
        let tx = token.transfer(wallet.address(), U256::from(1)).legacy();
        let rt: Result<_, _> = tx.send().await;
        match rt {
            Ok(info) => println!("prepare success"),
            Err(e) => println!("prepare fail: {:?}", e),
        }
        std::thread::sleep(Duration::from_secs(5));

        let singer = Arc::new(SignerMiddleware::new(l2_provider.clone(), wallet));
        let token_ts: Token<SignerMiddleware<Provider<Http>, _>> = Token::new(
            Address::from_str("0xdF2A58b54F0fd57C1bfd8aCe2F58711d50B52A61").unwrap(),
            singer,
        );
        token_vec.push(token_ts);
    }

    // token_vec.spl
    println!("current time: {:?}", SystemTime::now());
    println!(
        "block_number_start:{:?}",
        l2_provider.get_block_number().await.unwrap()
    );
    for i in 1..200 {
        for chunk in token_vec.chunks(2) {
            let mut handles: Vec<JoinHandle<()>> = Vec::new();
            for token_ts in chunk.to_owned() {
                let handle = tokio::spawn(async move {
                    let tx = token_ts
                        .transfer(Address::random(), U256::from(1))
                        .gas(60000)
                        .legacy();
                    let rt = tx.send().await;
                    match rt {
                        Ok(info) => println!("tx success"),
                        Err(e) => println!("tx fail: {:?}", e),
                    }
                });
                handles.push(handle);
            }
            std::thread::spawn(|| async {
                for h in handles {
                    h.await.unwrap();
                }
            });
            println!("chunk start");
            std::thread::sleep(Duration::from_millis(500));
        }
        println!("===========epoch:{:?} complete", i);
    }
    println!(
        "block_number_end:{:?}",
        l2_provider.get_block_number().await.unwrap()
    );
    println!("current time: {:?}", SystemTime::now());

    std::thread::sleep(Duration::from_secs(20));
}
