use loco_rs::{model::{ModelError, ModelResult}, prelude::model};
use sea_orm::entity::prelude::*;
use crate::models::_entities::apikeys;

pub use super::_entities::apikeys::{ActiveModel, Model, Entity};
pub type Apikeys = Entity;

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
    pub async fn find_by_userid(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let key = apikeys::Entity::find()
            .filter(
                model::query::condition()
                    .eq(apikeys::Column::UserId, id)
                    .build()
            )
            .one(db)
            .await?;
        key.ok_or_else(|| ModelError::EntityNotFound)
    }
    
    pub async fn verify_key(db: &DatabaseConnection, apikey: String) -> ModelResult<Self> {
        if apikey.is_empty() {
            return Err(ModelError::EntityNotFound);
        }
        let key = apikeys::Entity::find()
        .filter(
            model::query::condition()
            .eq(apikeys::Column::Value, &apikey)
            .build()
        )
        .one(db)
        .await?;

        key.ok_or_else(|| ModelError::EntityNotFound)
        
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
