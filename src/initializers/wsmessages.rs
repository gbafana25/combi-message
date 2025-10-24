use async_trait::async_trait;
use axum::routing::get;
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::extract::{AckSender, Data, Extension, SocketRef};
use socketioxide::SocketIo;
use axum::{Router as AxumRouter};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub struct WsMessageInitializer;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Username(String);

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
enum Res {
    Login {
        #[serde(rename = "numUsers")]
        num_users: usize,
    },
    UserEvent {
        #[serde(rename = "numUsers")]
        num_users: usize,
        username: Username,
    },
    Message {
        username: Username,
        message: String,
    },
    Username {
        username: Username,
    },
}

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

        io.ns("/", on_connect);

        let router = router.layer(
            ServiceBuilder::new()
                .layer(CorsLayer::very_permissive())
                .layer(layer),
        ).route("/", get(|| async {"connected"}));

        Ok(router)
    }

}