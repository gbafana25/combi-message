#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::{debug_handler, extract::Query};
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

use crate::models::{_entities::{apikeys, messages}, messages::{ActiveModel, Model}};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetPrivateParams {
    pub api_key: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetPrivateParams {
    pub key: String,
    pub value: String,
    pub api_key: String
}

impl SetPrivateParams {
    fn update(&self, item: &mut ActiveModel, device: String) {
        item.key = Set(Some(self.key.clone()));
        item.value = Set(Some(self.value.clone()));
        item.device_name = Set(Some(device));
        item.msg_id = Set(Uuid::new_v4());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetPublicParams {
    pub key: String,
    pub value: String
}

impl SetPublicParams {
    fn update(&self, item: &mut ActiveModel, device: String) {
        item.key = Set(Some(self.key.clone()));
        item.value = Set(Some(self.value.clone()));
        item.device_name = Set(Some(device));
        item.msg_id = Set(Uuid::new_v4());
    }
}

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub async fn load_item(ctx: &AppContext, device_name: String) -> Result<Model> { 
    let item = messages::Entity::find()
        .filter(messages::Column::DeviceName.eq(device_name))
        .filter(messages::Column::Isprivate.eq(0))
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

pub async fn list_public(device_name: String, ctx: &AppContext) -> Result<Response> {
    let res = messages::Entity::find()
        .filter(messages::Column::DeviceName.eq(device_name))
        .filter(messages::Column::Isprivate.eq(0))
        .all(&ctx.db).await?;
    format::json(res)
}

pub async fn list_all(device_name: String, ctx: &AppContext) -> Result<Response> {
    let res = messages::Entity::find()
        .filter(messages::Column::DeviceName.eq(device_name))
        .all(&ctx.db).await?;
    format::json(res)
}

pub async fn set_private(State(ctx): State<AppContext>, Path(device_name): Path<String>, Query(params): Query<SetPrivateParams>) -> Result<Response> {
    let paramsclone = params.clone();
    let keysearchval: String = paramsclone.key;
    let apikey: String = paramsclone.api_key;
    let Ok(message) = messages::Model::find_by_key(&ctx.db, &keysearchval, &device_name).await else {
        let mut activeitem: ActiveModel = Default::default();
        let Ok(_) = apikeys::Model::verify_key(&ctx.db, apikey).await else {
            return bad_request("Couldn't find message");
        };
        activeitem.isprivate = Set(Some(1));
        params.update(&mut activeitem, device_name);
        let item = activeitem.insert(&ctx.db).await?;
        return format::json(item);
    };
    let Ok(_) = apikeys::Model::verify_key(&ctx.db, apikey).await else {
        return bad_request("Couldn't find message");
    };
    let mut modified_item: ActiveModel = message.into();
    modified_item.value = Set(Some(params.value.to_owned()));
    let ret_item = modified_item.update(&ctx.db).await?;
    return format::json(ret_item);
}

pub async fn set_public(State(ctx): State<AppContext>, Path(device_name): Path<String>, Query(params): Query<SetPublicParams>) -> Result<Response> {
    let paramsclone = params.clone();
    let keysearchval: String = paramsclone.key;
    let Ok(message) = messages::Model::find_by_key(&ctx.db, &keysearchval, &device_name).await else {
        let mut activeitem: ActiveModel = Default::default();
        activeitem.isprivate = Set(Some(0));
        params.update(&mut activeitem, device_name);
        let item = activeitem.insert(&ctx.db).await?;
        return format::json(item);
    };

    let mut modified_item: ActiveModel = message.into();
    modified_item.value = Set(Some(params.value.to_owned()));
    let ret_item = modified_item.update(&ctx.db).await?;
    return format::json(ret_item);
}

pub async fn get_one(Path(device_name): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, device_name).await?)
}


pub async fn get_all(Path(device_name): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    list_public(device_name, &ctx).await
}

pub async fn get_all_with_private(Path(device_name): Path<String>, Query(api_key): Query<GetPrivateParams>, State(ctx): State<AppContext>) -> Result<Response> {

    match api_key.api_key {
        Some(akey) => {
            let Ok(_) = apikeys::Model::verify_key(&ctx.db, akey).await else {
                return bad_request("Invalid api key");
            };
            return list_all(device_name, &ctx).await;
        },
        None => {
            return list_public(device_name, &ctx).await;
        }
    }
  

}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/messages/")
        .add("/", get(index))
        .add("/set/private/{device_name}", get(set_private))
        .add("/set/{device_name}", get(set_public))
        .add("/get-latest/{device_name}", get(get_one))
        .add("/get/{device_name}", get(get_all_with_private)) // public + private
}
