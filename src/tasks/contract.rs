use loco_rs::prelude::*;
use alloy::{primitives::address, providers::ProviderBuilder, sol};
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "src/tasks/abi/IWETH.json"
);
pub struct Contract;
#[async_trait]
impl Task for Contract {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "Contract".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task Contract generated");
        let rpc_url = "https://eth.merkle.io".parse().map_err(|_e|Error::Message(String::from("Parse Rpc fail")))?;

        let provider = ProviderBuilder::new().on_http(rpc_url);
    
        // Create a contract instance.
        let contract = IWETH9::new(address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), provider);
    
        // Call the contract, retrieve the total supply.
        let total_supply = contract.totalSupply().call().await.map_err(|e|Error::Message(e.to_string()))?._0;
    
        println!("WETH total supply is {total_supply}");
    
        Ok(())
    }
}


