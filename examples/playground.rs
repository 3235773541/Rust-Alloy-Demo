// located in examples/playground.rs
// use this file to experiment with stuff
use loco_rs::{cli::playground, prelude::*};
// to refer to articles::ActiveModel, your imports should look like this:
use myapp::{app::App, models::_entities::articles};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    // add this:
    let active_model: articles::ActiveModel = articles::ActiveModel {
        title: Set(Some("how to build apps in 3 steps".to_string())),
        content: Set(Some("use Loco: https://loco.rs".to_string())),
        ..Default::default()
    };
    active_model.insert(&ctx.db).await.unwrap();

    let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    println!("{:?}", res);

    Ok(())
}
