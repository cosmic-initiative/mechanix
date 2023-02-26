mod singularity;

use axum::response::Html;
use axum::Router;
use axum::routing::get;
use crate::singularity::Singularity;

#[tokio::main]
async fn main() {

    let mut singularity = Singularity::new();
    singularity.start().await.unwrap();
}



async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
