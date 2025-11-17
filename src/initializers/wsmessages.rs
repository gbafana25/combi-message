use async_trait::async_trait;
use axum::http::HeaderValue;
use axum::routing::get;
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use axum::{Router as AxumRouter};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use crate::models::_entities::{apikeys, messages};
use crate::models::messages::ActiveModel;

pub struct WsMessageInitializer;

#[derive(Serialize, Deserialize)]
pub struct SetMessage {
    devicename: String,
    key: String,
    pub value: String,
    pub apikey: String
}

impl SetMessage {
    pub fn update(&self, item: &mut ActiveModel, device: String) {
        item.key = Set(Some(self.key.clone()));
        item.value = Set(Some(self.value.clone()));
        item.device_name = Set(Some(device));
        item.msg_id = Set(Uuid::new_v4());
    }
}

#[derive(Serialize, Deserialize)]
struct GetReq {
    devicename: String,
    apikey: String
}

#[derive(Serialize, Deserialize)]
struct Error {
    error: String
}

#[async_trait]
impl Initializer for WsMessageInitializer {
    fn name(&self) -> String {
        "websocket-messages".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let (layer, io) = SocketIo::builder()
            .build_layer();
        let ctcopy = ctx.clone();
        let c2 = ctx.clone();
        //let dbconn = ctcopy.db.to_owned();

        let corslayer = CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()).allow_headers(Any);

        io.ns("/", | socket: SocketRef | {
            
            socket.emit("connected", &GetReq {devicename: "test".to_string(), apikey: "".to_string()}).ok();

            socket.on(
                "set",
                async |socket: SocketRef, Data::<SetMessage>(data)| {
                    let dbcopy = c2;
                    match messages::Model::find_by_key(&dbcopy.db, &data.key, &data.devicename).await {
                        Ok(msg) => {
                            if msg.isprivate.unwrap() == 1 {
                                match apikeys::Model::verify_key(&dbcopy.db, &data.apikey).await {
                                    Ok(_) => {
                                        socket.emit("set-return", &messages::ActiveModel::update_item_ws(&dbcopy.db, data, msg).await).ok();
                                    },
                                    Err(_) => {
                                        socket.emit("error", "invalid api key").ok();
                                    }
                                    
                                };
                            } else {
                                socket.emit("set-return", &messages::ActiveModel::update_item_ws(&dbcopy.db, data, msg).await).ok();
                            }
                            
                        },
                        Err(_) => {
                            socket.emit("set-return", &messages::ActiveModel::create_item_ws(&dbcopy.db, data.devicename.clone(), data).await).ok();
                        }
                        
                        
                    };
                },
            );

            socket.on(
                "get",
                async |socket: SocketRef, Data::<GetReq>(data)| {
                    let dbcopy = ctcopy;
                    if !data.apikey.is_empty() {
                        match apikeys::Model::verify_key(&dbcopy.db, &data.apikey).await {
                            Ok(_) => {
                                socket.emit("get-return", &messages::Entity::get_all_ws(data.devicename.clone(), &dbcopy.db).await).ok();
                            },
                            Err(_) => {
                                socket.emit("error", "bad api key").ok();
                            }
                        }
                    } else {
                        socket.emit("get-return", &messages::Entity::get_public_ws(data.devicename.clone(), &dbcopy.db).await).ok();
                    }
                    
                }
            );
        });

        let router = router.layer(
            ServiceBuilder::new()
                .layer(corslayer)
                .layer(layer),
        ).route("/", get(|| async {"connected"}));

        Ok(router)
    }

}