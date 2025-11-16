#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::{debug_handler, extract::Query};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::models::{_entities::{apikeys, messages}, messages::ActiveModel};

#[derive(Debug, FromQueryResult, Serialize)]
pub struct ReturnMessageFormat {
    pub value: String,
    pub device_name: String,
    pub key: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

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
    pub fn update(&self, item: &mut ActiveModel, device: String) {
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
        return messages::ActiveModel::create_item(&ctx.db, device_name, params, isprivate, userid).await;
    };

    return messages::ActiveModel::update_item(&ctx.db, params, message).await;

}

pub async fn get_one(Path(device_name): Path<String>, Query(api_key): Query<GetPrivateParams>, State(ctx): State<AppContext>) -> Result<Response> {
    match api_key.api_key {
        Some(akey) => {
            let Ok(_) = apikeys::Model::verify_key(&ctx.db, &akey).await else {
                return bad_request("Invalid api key");
            };
            return format::json(messages::Entity::load_item(&ctx.db, device_name, 1).await?);
            
        },
        None => {
            return format::json(messages::Entity::load_item(&ctx.db, device_name, 0).await?);
        }
    }
    
}

pub async fn get_all_with_private(Path(device_name): Path<String>, Query(api_key): Query<GetPrivateParams>, State(ctx): State<AppContext>) -> Result<Response> {

    match api_key.api_key {
        Some(akey) => {
            let Ok(a) = apikeys::Model::verify_key(&ctx.db, &akey).await else {
                return bad_request("Invalid api key");
            };
            return messages::Entity::list_all(device_name, a.user_id, &ctx.db).await;
        },
        None => {
            return messages::Entity::list_public(device_name, &ctx.db).await;
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
