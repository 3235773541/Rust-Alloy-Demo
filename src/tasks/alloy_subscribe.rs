use loco_rs::prelude::*;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde_json::json;
use url::Url;


pub struct AlloySubscribe;

#[async_trait]
impl Task for AlloySubscribe {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "alloy_subscribe".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let url = Url::parse("wss://eth-mainnet.g.alchemy.com/v2/wVrJATi4oMUtUlEWKyM-dTOxeMZ4TEHO").expect("Failed to parse URL");
        let (mut ws_stream, _) = connect_async(url.as_str()).await.expect("Failed to connect");
        println!("WebSocket client connected");


         // 创建订阅消息
         let subscribe_message = json!({
             "jsonrpc": "2.0",
             "id": 1,
             "method": "eth_subscribe",
             "params": ["newPendingTransactions"],
         }).to_string();
     
         ws_stream.send(Message::Text(subscribe_message)).await.expect("Failed to send subscribe message");
     
         // 处理接收到的消息
         while let Some(message) = ws_stream.next().await {
             match message {
                 Ok(Message::Text(text)) => {
                     println!("Received message: {}", text);
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
