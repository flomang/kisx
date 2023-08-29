use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{models, utils::auth::Auth};

// Client Messages ↓

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLot {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub category: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub condition: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub title: String,
    pub external_id: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: String,
    pub images: Vec<CreateLotImage>,
    pub meta_data: serde_json::Value,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLot {
    #[validate(custom(function = "validate_uuid", message = "lot id must be uuid"))]
    pub lot_id: String,
    pub category: Option<String>,
    pub condition: Option<String>,
    pub title: Option<String>,
    pub external_id: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    //pub new_images: ...,
    // vec of image uuids to delete
    pub deleted_image_ids: Vec<String>,
}

fn validate_uuid(lot_id: &str) -> Result<(), ValidationError> {
    uuid::Uuid::parse_str(lot_id)
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_uuid"))
}

// convert client message to db message
impl From<UpdateLot> for models::UpdateLot {
    fn from(lot: UpdateLot) -> Self {
        let id = Uuid::try_parse(&lot.lot_id).unwrap();

        models::UpdateLot {
            id,
            category: lot.category,
            condition: lot.condition,
            title: lot.title,
            external_id: lot.external_id,
            description: lot.description,
            status: lot.status,
        }
    }
}

#[derive(Debug)]
pub struct CreateLotAuthenticated {
    pub auth: Auth,
    pub lot: CreateLot,
}

#[derive(Debug)]
pub struct UpdateLotAuthenticated {
    pub auth: Auth,
    pub lot: models::UpdateLot,
}

#[derive(Debug)]
pub struct DeleteLotAuthenticated {
    pub auth: Auth,
    pub lot_id: Uuid,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLotImage {
    pub image_url: String,
    pub is_thumbnail: bool,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterLots {
    pub categories: Vec<String>,
    pub conditions: Vec<String>,
    pub terms: Vec<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub statuses: Vec<String>,
}

#[derive(Debug)]
pub struct FilterLotsAuthenticated {
    pub auth: Auth,
    pub params: FilterLots,
    pub owner_id: Option<Uuid>,
}

// Server Responses ↓
