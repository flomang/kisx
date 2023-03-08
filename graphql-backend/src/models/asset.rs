use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::assets;

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[diesel(table_name = assets)]
pub struct Asset {
    pub id: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub meta_data: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = assets)]
pub struct NewAsset {
    pub id: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = assets)]
pub struct UpdateAsset {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
}