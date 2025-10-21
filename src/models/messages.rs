use loco_rs::model::{self, ModelError, ModelResult};
use sea_orm::entity::prelude::*;
use crate::models::_entities::messages;

pub use super::_entities::messages::{ActiveModel, Model, Entity};
pub type Messages = Entity;

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
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
