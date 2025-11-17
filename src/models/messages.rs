use loco_rs::{config::Database, model::{self, ModelError, ModelResult}};
use sea_orm::{FromQueryResult, QueryOrder, QuerySelect, entity::prelude::*};
use serde::Serialize;
use crate::{controllers::messages::SetPrivateParams, initializers::wsmessages::SetMessage, models::_entities::{apikeys, messages}};
use loco_rs::prelude::*;

pub use super::_entities::messages::{ActiveModel, Model, Entity};
pub type Messages = Entity;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct ReturnMessageFormat {
    pub value: String,
    pub device_name: String,
    pub key: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn find_by_key(db: &DatabaseConnection, key: &str, dev: &str) -> ModelResult<Self> {
        let message = messages::Entity::find()
            .filter(
                model::query::condition()
                .eq(messages::Column::Key, key)
                .eq(messages::Column::DeviceName, dev)
                .build()
            )
            .one(db)
            .await?;
        message.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create_item(db: &DatabaseConnection, device_name: String, params: SetPrivateParams, isprivate: i32, userid: i32) -> Result<Response> {
        let mut activeitem: ActiveModel = Default::default();
        params.update(&mut activeitem, device_name);   
        activeitem.isprivate = Set(Some(isprivate));
        activeitem.user_id = Set(userid);
        let item = activeitem.insert(db).await?;
        return format::json(item);
    }

    pub async fn update_item(db: &DatabaseConnection, params: SetPrivateParams, message: Model) -> Result<Response> {
        let mut modified_item: ActiveModel = message.into();
        modified_item.value = Set(Some(params.value.to_owned()));
        let ret_item = modified_item.update(db).await?;
        return format::json(ret_item);
    }

    pub async fn create_item_ws(db: &DatabaseConnection, device_name: String, data: SetMessage) -> Model {
        let mut activeitem: ActiveModel = Default::default();
        data.update(&mut activeitem, device_name);
        if data.apikey.is_empty() {
            activeitem.isprivate = Set(Some(0));
            activeitem.user_id = Set(0);
        } else {
            
            match apikeys::Model::verify_key(db, &data.apikey).await {
                Ok(akey) => {
                    activeitem.isprivate = Set(Some(1));
                    activeitem.user_id = Set(akey.user_id);
                },
                Err(_) => {
                    
                }
            }
        }
        
        return activeitem.insert(db).await.unwrap();
    }

    pub async fn update_item_ws(db: &DatabaseConnection, data: SetMessage, msg: Model) -> Model {
        let mut modified_item: ActiveModel = msg.into();
        modified_item.value = Set(Some(data.value));
        return modified_item.update(db).await.unwrap();
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn load_item(db: &DatabaseConnection, device_name: String, isprivate: i32) -> Result<ReturnMessageFormat> { 
        let item = messages::Entity::find()
            .select_only()
            .columns([messages::Column::CreatedAt, messages::Column::DeviceName, messages::Column::Key, messages::Column::Value, messages::Column::UpdatedAt])
            .filter(messages::Column::DeviceName.eq(device_name))
            .filter(messages::Column::Isprivate.eq(isprivate))
            .order_by(messages::Column::UpdatedAt, sea_orm::Order::Desc)
            .into_model::<ReturnMessageFormat>()
            .one(db).await?;
        item.ok_or_else(|| Error::NotFound)
    }

    pub async fn list_all(device_name: String, user_id: i32, db: &DatabaseConnection) -> Result<Response> {
        let res = messages::Entity::find()
            .select_only()
            .columns([messages::Column::CreatedAt, messages::Column::DeviceName, messages::Column::Key, messages::Column::Value, messages::Column::UpdatedAt])
            .filter(messages::Column::DeviceName.eq(device_name))
            .filter(messages::Column::UserId.eq(user_id).or(messages::Column::UserId.eq(0)))
            .into_model::<ReturnMessageFormat>()
            .all(db).await?;
        format::json(res)
    }

    pub async fn list_public(device_name: String, db: &DatabaseConnection) -> Result<Response> {
        let res = messages::Entity::find()
            .select_only()
            .columns([messages::Column::CreatedAt, messages::Column::DeviceName, messages::Column::Key, messages::Column::Value, messages::Column::UpdatedAt])
            .filter(messages::Column::DeviceName.eq(device_name))
            .filter(messages::Column::Isprivate.eq(0))
            .into_model::<ReturnMessageFormat>()
            .all(db).await?;
        format::json(res)
    }

    pub async fn get_all_ws(device_name: String, db: &DatabaseConnection) -> Vec<Model> {
        return messages::Entity::find()
            .filter(messages::Column::DeviceName.eq(device_name))
            .all(db).await.unwrap();
    }

    pub async fn get_public_ws(device_name: String, db: &DatabaseConnection) -> Vec<Model> {
        return messages::Entity::find()
            .filter(messages::Column::DeviceName.eq(device_name))
            .filter(messages::Column::Isprivate.eq(0))
            .all(db).await.unwrap();
    }
}
