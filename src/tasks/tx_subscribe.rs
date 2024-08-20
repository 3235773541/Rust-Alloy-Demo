use futures::stream::StreamExt;
use loco_rs::prelude::*;
use web3::transports::WebSocket;
use web3::Web3;
use std::{env, process};
use std::env::VarError;
use dotenv::dotenv;
pub struct TxSubscribe;

fn get_node_endpoint() -> Result<String, VarError> {
    env::var("WSS_NODE_ENDPOINT")
}

#[async_trait]
impl Task for TxSubscribe {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "tx_subscribe".to_string(),
            detail: "Subscribe to all blockchain transactions and print them".to_string(),
        }
    }

    async fn run(&self, _app_context: &AppContext, _vars: &loco_rs::task::Vars) -> Result<()> {
        println!("Starting the task");
        dotenv().ok();
        let wss_node_endpoint = match get_node_endpoint() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error getting WSS_NODE_ENDPOINT: {:?}", e);
                process::exit(1);
            }
        };
        let websocket =
            WebSocket::new(wss_node_endpoint.as_str())
                .await
                .map_err(|e| Error::Message(e.to_string()))?;
        let web3 = Web3::new(websocket);
        let mut sub = web3
            .eth_subscribe()
            .subscribe_new_pending_transactions()
            .await
            .map_err(|e| Error::Message(e.to_string()))?;

        while let Some(tx_hash) = sub.next().await {
            println!("hash{:?}", tx_hash);
            match tx_hash {
                Ok(hash) => {
                    if let Ok(transaction) = web3
                        .eth()
                        .transaction(web3::types::TransactionId::from(hash))
                        .await
                    {
                        if let Some(tx) = transaction {
                            println!("{:?}", tx);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving transaction hash: {:?}", e);
                }
            }
        }

        Ok(())
    }
}
