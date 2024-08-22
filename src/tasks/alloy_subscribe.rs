use alloy::json_abi::JsonAbi;
use alloy::{
    sol, sol_types::SolCall,
};
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use loco_rs::prelude::*;
use serde_json::{json, Value};
use std::{env, fs::File, io::Read};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

pub struct AlloySubscribe;
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniversalRouter,
    "src/tasks/abi/UniversalRouter.json"
);

sol!(
    #[allow(missing_docs)]
    function execute(bytes calldata commands, bytes[] calldata inputs, uint256 deadline) external payable;
);

#[async_trait]
impl Task for AlloySubscribe {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "alloy_subscribe".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        dotenv().ok();
        let wss_url =
            env::var("WSS_NODE_ENDPOINT").expect("Alchemy WSS_URL environment variable not set");
        let url = Url::parse(&wss_url).expect("Failed to parse URL");
        let (mut ws_stream, _) = connect_async(url.as_str())
            .await
            .expect("Failed to connect");
        println!("WebSocket client connected");

        // Create a JSON-RPC subscribe message
        let subscribe_message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_subscribe",
            "params": ["alchemy_pendingTransactions"],
        })
        .to_string();

        ws_stream
            .send(Message::Text(subscribe_message))
            .await
            .expect("Failed to send subscribe message");

        // Process incoming messages
        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    let parsed_json: Value = serde_json::from_str(&text)?;
                    let from = parsed_json["params"]["result"]["from"]
                        .as_str()
                        .unwrap_or("Unknown");
                    let to = parsed_json["params"]["result"]["to"]
                        .as_str()
                        .unwrap_or("Unknown");
                    let value = parsed_json["params"]["result"]["value"]
                        .as_str()
                        .unwrap_or("Unknown");
                    let data = parsed_json["params"]["result"]["input"]
                        .as_str()
                        .unwrap_or("Unknown");

                    // println!("From:{}, To:{}, Value:{}, Data:{}", from, to, value, data);

                    //Check if the transaction is to the UniversalRouter contract
                    if to.to_lowercase()
                        == "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD".to_lowercase()
                    {
                        // println!("From:{}, To:{}, Value:{}, Data:{}", from, to, value, data);
                        let data = hex::decode(&data[2..]).expect("Failed to decode input data");

                        let decoded = UniversalRouter::execute_1Call::abi_decode(&data, false);
                        match decoded {
                            Ok(decoded) => {
                                let commands = decoded.commands;
                                let inputs = decoded.inputs;
                                let deadline = decoded.deadline;

                                println!(
                                    "UniversalRouter.execute Call Details:\nCommands: {:?}\nInputs: {:?}\nDeadline: {:?}",
                                    commands, inputs, deadline
                                );
                                // TODO: Decode the inputs and commands
                            }
                            Err(e) => {
                                println!("Error decoding input: {e:?}");
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    println!("WebSocket connection closed.");
                    break;
                }
                Err(e) => {
                    println!("WebSocket error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// load the ABI from a file
fn load_abi_from_file(path: &str) -> JsonAbi {
    let mut file = File::open(path).expect("Failed to open ABI file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read ABI file");

    serde_json::from_str(&contents).expect("Invalid ABI JSON format")
}

