use serde::Serialize;
use socketioxide::extract::AckSender;
use tracing::info;

#[derive(Serialize)]
pub struct HandResponse {
    cards: Vec<i32>,
}

pub fn hand(ack: AckSender) {
    info!("Requested hand");

    ack.send(HandResponse { cards: vec![4, 2] }).ok();
}
