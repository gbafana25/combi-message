use async_trait::async_trait;
use axum::http::HeaderValue;
use axum::routing::get;
use loco_rs::app::{AppContext, Initializer};
use loco_rs::Result;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use axum::{Router as AxumRouter};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use crate::models::_entities::messages;
use crate::models::messages::ActiveModel;

pub struct WsMessageInitializer;

#[derive(Serialize, Deserialize)]
struct Message {
    devicename: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct SetMessage {
    devicename: String,
    key: String,
    value: String
}

impl SetMessage {
    fn update(&self, item: &mut ActiveModel, device: String) {
        item.key = Set(Some(self.key.clone()));
        item.value = Set(Some(self.value.clone()));
        item.device_name = Set(Some(device));
        item.msg_id = Set(Uuid::new_v4());
    }
}

#[derive(Serialize, Deserialize)]
struct GetReq {
    devicename: String
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>, ctx: &AppContext) {
    socket.emit("connected", &data).ok();

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

        let corslayer = CorsLayer::new().allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());

        io.ns("/", | socket: SocketRef | {
            
            socket.emit("connected", &Message {devicename: "test".to_string(), value: "test".to_string()}).ok();

            socket.on(
                "set",
                async |socket: SocketRef, Data::<SetMessage>(data)| {
                    let dbcopy = c2;
                    match messages::Model::find_by_key(&dbcopy.db, &data.key, &data.devicename).await {
                        Ok(msg) => {
                            let mut modified_item: ActiveModel = msg.into();
                            modified_item.value = Set(Some(data.value));
                            let ret_item = modified_item.update(&dbcopy.db).await.unwrap();
                            socket.emit("set-return", &ret_item).ok();
                        },
                        Err(_) => {
                            let mut activeitem: ActiveModel = Default::default();
                            data.update(&mut activeitem, data.devicename.clone());
                            activeitem.isprivate = Set(Some(0));
                            let item = activeitem.insert(&dbcopy.db).await.unwrap();
                            socket.emit("set-return", &item).ok();
                        }
                        
                        
                    };
                },
            );

            socket.on(
                "get",
                async |socket: SocketRef, Data::<GetReq>(data)| {
                    let dbcopy = ctcopy;
                    let res = messages::Entity::find()
                        .filter(messages::Column::DeviceName.eq(data.devicename.clone()))
                        .filter(messages::Column::Isprivate.eq(0))
                        .all(&dbcopy.db).await.unwrap();
                    println!("Received get command: {:?}", res.get(0));
                    socket.emit("get-return", &res).ok();
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