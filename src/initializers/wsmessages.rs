use async_trait::async_trait;
use axum::http::HeaderValue;
use axum::routing::get;
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, SocketRef};
use socketioxide::SocketIo;
use axum::{Router as AxumRouter};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub struct WsMessageInitializer;

#[derive(Serialize, Deserialize)]
struct Message {
    key: String,
    value: String,
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    socket.emit("connected", &data).ok();

    socket.on(
        "set",
        |socket: SocketRef, Data::<Message>(data)| {
            println!("Received set command: {}, {}", data.key, data.value);
            socket.emit("set-return", &data).ok();
        },
    );

    socket.on(
        "get",
        |Data::<Value>(data), ack: AckSender| {
            ack.send(&data).ok();
        }
    );
}

#[async_trait]
impl Initializer for WsMessageInitializer {
    fn name(&self) -> String {
        "websocket-messages".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let (layer, io) = SocketIo::builder()
            .build_layer();

        let corslayer = CorsLayer::new().allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());

        io.ns("/", on_connect);

        let router = router.layer(
            ServiceBuilder::new()
                .layer(corslayer)
                .layer(layer),
        ).route("/", get(|| async {"connected"}));

        Ok(router)
    }

}