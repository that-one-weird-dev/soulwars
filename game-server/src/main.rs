use socketioxide::SocketIo;
use state::game_state::GameState;
use tracing_subscriber::FmtSubscriber;

mod handlers;
mod state;
mod game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber)?;

    let (layer, io) = SocketIo::builder()
        .with_state(GameState::default())
        .build_layer();

    io.ns("/", handlers::connection::handle_connection);

    let app = axum::Router::new()
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4890").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
