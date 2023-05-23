use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{Lot, LotImage},
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
    // pub images: serde_json::Value,
    pub images: Vec<CreateLotImage>,
    pub meta_data: serde_json::Value,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLotImage {
    pub image_url: String,
    pub is_thumbnail: bool,
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
    pub meta_data: Option<serde_json::Value>,
    //pub images: Vec<LotImageInner>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LotImageInner {
    pub id: String,
    pub image_url: String,
    pub is_thumbnail: bool,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct LotResponse {
    pub lot: LotInner,
    pub images: Vec<LotImageInner>,
}

impl From<LotImage> for LotImageInner {
    fn from(image: LotImage) -> Self {
        LotImageInner {
            id: image.id.to_string(),
            image_url: image.image_url,
            is_thumbnail: image.is_thumbnail,
            created_at: CustomDateTime(image.created_at),
            updated_at: CustomDateTime(image.updated_at),
        }
    }
}

impl From<Lot> for LotInner {
    fn from(lot: Lot) -> Self {
        LotInner {
            id: lot.id.to_string(),
            user_id: lot.user_id.to_string(),
            category: lot.category,
            condition: lot.condition,
            tag: lot.tag,
            description: lot.description,
            meta_data: lot.meta_data,
            created_at: CustomDateTime(lot.created_at),
            updated_at: CustomDateTime(lot.updated_at),
        }
    }
}

// impl From<Lot> for LotResponse {
//     fn from(lot: Lot) -> Self {
//         // let images = lot.images.into_iter().map(|image| {
//         //     LotImageInner {
//         //         id: image.id.to_string(),
//         //         image_url: image.image_url,
//         //         is_thumbnail: image.is_thumbnail,
//         //         created_at: CustomDateTime(image.created_at),
//         //         updated_at: CustomDateTime(image.updated_at),
//         //     }
//         // }).collect();

//         LotResponse {
//             lot: LotInner {
//                 id: lot.id.to_string(),
//                 user_id: lot.user_id.to_string(),
//                 category: lot.category,
//                 condition: lot.condition,
//                 tag: lot.tag,
//                 description: lot.description,
//                 // images: images,
//                 meta_data: lot.meta_data,
//                 created_at: CustomDateTime(lot.created_at),
//                 updated_at: CustomDateTime(lot.updated_at),
//             },
//         }
//     }
// }
