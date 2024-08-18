#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;


#[debug_handler]
pub async fn echo(req_body: String) -> String {
    req_body
}

#[debug_handler]
pub async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
    // do something with context (database, etc)
    format::text("hello")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("guide")
        .add("/", get(hello))
        .add("/echo", post(echo))
}
