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
    pub api_key: Option<String>
}

impl SetPrivateParams {
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

pub async fn load_item(ctx: &AppContext, device_name: String, isprivate: i32) -> Result<Model> { 
    let item = messages::Entity::find()
        .filter(messages::Column::DeviceName.eq(device_name))
        .filter(messages::Column::Isprivate.eq(isprivate))
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

pub async fn list_all(device_name: String, user_id: i32, ctx: &AppContext) -> Result<Response> {
    let res = messages::Entity::find()
        .filter(messages::Column::DeviceName.eq(device_name))
        .filter(messages::Column::UserId.eq(user_id).or(messages::Column::UserId.eq(0)))
        .all(&ctx.db).await?;
    format::json(res)
}

pub async fn set(State(ctx): State<AppContext>, Path(device_name): Path<String>, Query(params): Query<SetPrivateParams>) -> Result<Response> {
    let paramsclone = params.clone();
    let keysearchval: String = paramsclone.key;
    let mut isprivate = 0;
    let mut userid = 0;
    // verify key and set isprivate variable
    match params.api_key {
        Some(ref akey) => {
            let Ok(a) = apikeys::Model::verify_key(&ctx.db, akey).await else {
                return bad_request("Invalid API key");
            };
            isprivate = 1;
            userid = a.user_id;
        },
        None => {
            // public
            isprivate = 0;
            userid = 0;
        }
    }

    // message already exists
    let Ok(message) = messages::Model::find_by_key(&ctx.db, &keysearchval, &device_name).await else {
        let mut activeitem: ActiveModel = Default::default();
        params.update(&mut activeitem, device_name);   
        activeitem.isprivate = Set(Some(isprivate));
        activeitem.user_id = Set(userid);
        let item = activeitem.insert(&ctx.db).await?;
        return format::json(item);

        
    };

    let mut modified_item: ActiveModel = message.into();
    modified_item.value = Set(Some(params.value.to_owned()));
    let ret_item = modified_item.update(&ctx.db).await?;
    return format::json(ret_item);

}

pub async fn get_one(Path(device_name): Path<String>, Query(api_key): Query<GetPrivateParams>, State(ctx): State<AppContext>) -> Result<Response> {
    match api_key.api_key {
        Some(akey) => {
            let Ok(_) = apikeys::Model::verify_key(&ctx.db, &akey).await else {
                return bad_request("Invalid api key");
            };
            return format::json(load_item(&ctx, device_name, 1).await?);
            
        },
        None => {
            return format::json(load_item(&ctx, device_name, 0).await?);
        }
    }
    
}


pub async fn get_all(Path(device_name): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    list_public(device_name, &ctx).await
}

pub async fn get_all_with_private(Path(device_name): Path<String>, Query(api_key): Query<GetPrivateParams>, State(ctx): State<AppContext>) -> Result<Response> {

    match api_key.api_key {
        Some(akey) => {
            let Ok(a) = apikeys::Model::verify_key(&ctx.db, &akey).await else {
                return bad_request("Invalid api key");
            };
            return list_all(device_name, a.user_id, &ctx).await;
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
        .add("/set/{device_name}", get(set))
        .add("/get-latest/{device_name}", get(get_one))
        .add("/get/{device_name}", get(get_all_with_private)) // public + private
}
