use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;

use crate::entity;
use crate::Error;
use crate::ErrorType;
use crate::Result;

#[derive(Debug)]
pub struct Friend {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub avatar_url: String,
    pub url: String,
}

impl Friend {
    pub async fn get_list(conn: &DatabaseConnection) -> Result<Vec<Friend>> {
        let friends = entity::Friend::find().all(conn).await?;

        let friends = friends
            .into_iter()
            .map(|friend| Friend::from(friend))
            .collect();

        Ok(friends)
    }

    pub async fn insert(conn: &DatabaseConnection, friend: Friend) -> Result<i32> {
        let friend = entity::friend::ActiveModel {
            id: Set(friend.id),
            name: Set(friend.name),
            description: Set(friend.description),
            avatar_url: Set(friend.avatar_url),
            url: Set(friend.url),
        };

        let res = entity::Friend::insert(friend).exec(conn).await?;

        Ok(res.last_insert_id)
    }

    pub async fn update(conn: &DatabaseConnection, friend: Friend) -> Result<i32> {
        let mut act_model: entity::friend::ActiveModel = entity::Friend::find_by_id(friend.id)
            .one(conn)
            .await?
            .ok_or(Error::new(
                "Friend",
                "Friend not found",
                ErrorType::TagNotFound,
            ))?
            .into();

        act_model.name = Set(friend.name);
        act_model.description = Set(friend.description);
        act_model.avatar_url = Set(friend.avatar_url);

        let friend = entity::Friend::update(act_model).exec(conn).await?;

        Ok(friend.id)
    }

    pub async fn delete(conn: &DatabaseConnection, id: i32) -> Result<()> {
        entity::Friend::delete_by_id(id).exec(conn).await?;

        Ok(())
    }
}

impl From<entity::FriendModel> for Friend {
    fn from(friend: entity::FriendModel) -> Self {
        Friend {
            id: friend.id,
            name: friend.name,
            description: friend.description,
            avatar_url: friend.avatar_url,
            url: friend.url,
        }
    }
}
