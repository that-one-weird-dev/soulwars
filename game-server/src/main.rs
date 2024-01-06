use socketioxide::SocketIo;
use tracing_subscriber::FmtSubscriber;
use user_state::UserState;

mod handlers;
mod user_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber)?;

    let (layer, io) = SocketIo::builder()
        .with_state(UserState::default())
        .build_layer();

    io.ns("/", handlers::connection::handle_connection);

    let app = axum::Router::new()
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4890").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
