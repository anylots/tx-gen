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
    let tx = TransactionRequest::new()
        .to(wallet1.address())
        .value(1 * 10u64.pow(18));
    l2_signer.send_transaction(tx, None).await.unwrap();
    std::thread::sleep(Duration::from_secs(2));

    let tx1 = token.transfer(wallet1.address(), U256::from(100000)).legacy();
    let rt: Result<_, _> = tx1.send().await;
    std::thread::sleep(Duration::from_secs(2));

    let l2_signer1 = Arc::new(SignerMiddleware::new(l2_provider.clone(), wallet1));

    let mut token_vec = Vec::<Token<SignerMiddleware<Provider<Http>, _>>>::new();
    let mut i: i32 = 0;
    while i < 10 {
        i += 1;
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let wallet = Wallet::new(&mut rng).with_chain_id(chain_id);
        let tx = TransactionRequest::new()
            .to(wallet.address())
            .value(1 * 10u64.pow(16));
        l2_signer1.send_transaction(tx, None).await.unwrap();
        std::thread::sleep(Duration::from_secs(2));

        //Prepare balance
        let tx1 = token.transfer(wallet.address(), U256::from(1000)).legacy();
        let rt: Result<_, _> = tx1.send().await;
        let pending_tx = match rt {
            Ok(pending_tx) => pending_tx,
            Err(e) => {
                println!("prepare fail: {:?}", e);
                continue;
            }
        };
        std::thread::sleep(Duration::from_secs(2));
        println!(
            "==========>Prepare balance block_number:{:?}",
            l2_provider.get_block_number().await.unwrap()
        );

        let receipt = l2_provider
            .get_transaction_receipt(pending_tx.tx_hash())
            .await
            .unwrap();

        match receipt {
            Some(receipt) => {
                match receipt.status.unwrap().as_u64() {
                    1 => println!("prepare success:{:?}", receipt),
                    _ => {
                        println!("prepare fail");
                        continue;
                    }
                };
            }
            // Maybe still pending
            None => {
                println!("prepare pending");
                // continue;
            }
        }
        std::thread::sleep(Duration::from_millis(2000));

        let balance = token.balance_of(wallet.address()).await.unwrap();
        println!("balance: {:?}", balance);
        assert!(balance == U256::from(1000), "Balance is not as expected");

        let singer = Arc::new(SignerMiddleware::new(l2_provider.clone(), wallet));
        let token_ts: Token<SignerMiddleware<Provider<Http>, _>> =
            Token::new(Address::from_str(token_address.as_str()).unwrap(), singer);
        token_vec.push(token_ts);
    }

    // token_vec.spl
    println!("waiting for start erc20 tx");
    std::thread::sleep(Duration::from_secs(10));
    println!("current time: {:?}", SystemTime::now());
    println!(
        "==========>block_number_start:{:?}",
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

        println!(
            "===========epoch: {:?} complete, block_number: {:?}",
            i,
            l2_provider.get_block_number().await.unwrap()
        );
    }
    println!(
        "==========>block_number_end:{:?}",
        l2_provider.get_block_number().await.unwrap()
    );
    println!("current time: {:?}", SystemTime::now());

    std::thread::sleep(Duration::from_secs(20));
}
