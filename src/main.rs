use axum::response::Html;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let std_listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    std_listener.set_nonblocking(true).unwrap();
    axum::Server::from_tcp(std_listener)
        .unwrap()
        .serve(app.into_make_service()).await.unwrap();
}



async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
