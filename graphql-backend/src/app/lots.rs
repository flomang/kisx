use uuid::Uuid;
use validator::Validate;

use crate::{
    models::Lot,
    utils::{auth::Auth, CustomDateTime},
};

// Client Messages ↓

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLot {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub category: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub condition: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub tag: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: String,
    pub images: serde_json::Value,
    pub data: serde_json::Value,
}

#[derive(Debug)]
pub struct CreateLotOuter {
    pub auth: Auth,
    pub lot: CreateLot,
}

// Server Responses ↓

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LotInner {
    pub id: String,
    pub user_id: String,
    pub category: String,
    pub condition: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub data: Option<serde_json::Value>,
    pub images: Option<serde_json::Value>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct LotResponse {
    //pub lot: Lot,
    pub lot: LotInner,
}

impl From<Lot> for LotResponse {
    fn from(lot: Lot) -> Self {
        LotResponse {
            lot: LotInner {
                id: lot.id.to_string(),
                user_id: lot.user_id.to_string(),
                category: lot.category,
                condition: lot.condition,
                tag: lot.tag,
                description: lot.description,
                images: lot.images,
                data: lot.data,
                created_at: CustomDateTime(lot.created_at),
                updated_at: CustomDateTime(lot.updated_at),
            },
        }
    }
}
