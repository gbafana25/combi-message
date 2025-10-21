#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

use crate::models::{_entities::{apikeys, messages}, messages::{ActiveModel, Entity, Model}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub device_name: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub api_key: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.key = Set(self.key.clone());
        item.value = Set(self.value.clone());
        item.device_name = Set(self.device_name.clone());
        item.msg_id = Set(Uuid::new_v4());
    }
}

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub async fn load_item(ctx: &AppContext, device_name: String) -> Result<Model> { 
    let item = messages::Entity::find().filter(
        messages::Column::DeviceName.eq(device_name))
        .order_by(messages::Column::CreatedAt, sea_orm::Order::Desc)
        .one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn load_from_key(ctx: &AppContext, key: String) -> Result<Model> {
    let item = messages::Entity::find().filter(
        messages::Column::Key.eq(key))
        .one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(device_name: String, ctx: &AppContext) -> Result<Response> {
    let res = messages::Entity::find().filter(
        messages::Column::DeviceName.eq(device_name)
    ).all(&ctx.db).await?;
    format::json(res)
}

pub async fn set(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let paramsclone = params.clone();
    let keysearchval: String = paramsclone.key.unwrap();
    let deviceval: String = paramsclone.device_name.unwrap();
    let apikey: String = paramsclone.api_key.unwrap();
    let Ok(message) = messages::Model::find_by_key(&ctx.db, &keysearchval, &deviceval).await else {
        let mut activeitem: ActiveModel = Default::default();
        if !apikey.is_empty() && apikeys::Model::verify_key(&ctx.db, apikey).await {
            activeitem.isprivate = Set(Some(1));
        } else {
            activeitem.isprivate = Set(Some(0));
        }
        params.update(&mut activeitem);
        let item = activeitem.insert(&ctx.db).await?;
        return format::json(item);
    };

    //message.value = params.value;
    let mut modified_item: ActiveModel = message.into();
    modified_item.value = Set(params.value.to_owned());
    let ret_item = modified_item.update(&ctx.db).await?;
    return format::json(ret_item);
    
}

pub async fn get_one(Path(device_name): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, device_name).await?)
}

pub async fn get_by_key(Path(key): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_from_key(&ctx, key).await?)
}


pub async fn get_all(Path(device_name): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    list(device_name, &ctx).await
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/messages/")
        .add("/", get(index))
        .add("/set", post(set))
        .add("/get/{key}", get(get_by_key))
        .add("/get-latest/{device_name}", get(get_one))
        .add("/get-all/{device_name}", get(get_all))
}
