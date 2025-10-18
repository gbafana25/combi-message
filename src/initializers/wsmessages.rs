use async_trait::async_trait;
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, Extension, SocketRef};
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

#[async_trait]
impl Initializer for WsMessageInitializer {
    fn name(&self) -> String {
        "websocket-messages".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let (layer, io) = SocketIo::builder()
            .build_layer();

        io.ns("/", |s: SocketRef| {
            s.on(
                "new message",
                |s: SocketRef, Data::<String>(msg), Extension::<Username>(username)| {
                    let msg = &Res::Message {
                        username,
                        message: msg,
                    };
                    s.broadcast().emit("new message", msg).ok();
                },
            );

            s.on("typing", |s: SocketRef, Extension::<Username>(username)| {
                s.broadcast()
                    .emit("typing", &Res::Username { username })
                    .ok();
            });

            s.on(
                "stop typing",
                |s: SocketRef, Extension::<Username>(username)| {
                    s.broadcast()
                        .emit("stop typing", &Res::Username { username })
                        .ok();
                },
            );

            s.on_disconnect(
                |s: SocketRef, Extension::<Username>(username)| {
                    s.broadcast().emit("user left", &username).ok();
                },
            );
        });

        let router = router.layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

        Ok(router)
    }

}