use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{lots::{self}, lot_images};

#[derive(Debug, Queryable, Identifiable)]
pub struct Lot {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub condition: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub meta_data: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(belongs_to(Lot))]
pub struct LotImage {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub image_url: String,
    pub is_thumbnail: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = lots)]
pub struct NewLot {
    pub user_id: Uuid,
    pub category: String,
    pub condition: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub meta_data: serde_json::Value,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = lot_images)]
pub struct NewLotImage {
    pub lot_id: Uuid,
    pub image_url: String,
    pub is_thumbnail: bool,
}