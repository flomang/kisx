use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::lots::{self};

#[derive(Debug, Queryable, Identifiable)]
pub struct Lot {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub condition: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub images: Option<serde_json::Value>,
    pub data: Option<serde_json::Value>,
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
    pub images: serde_json::Value,
    pub data: serde_json::Value,
}
