use crate::models::Asset;
use validator::Validate;

// user input adding a new asset to the user's portfolio
#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct AddAssetRequest {
    #[validate(length(
        min = 3,
        max = 255,
        message = "fails validation - must be 3-255 characters long"
    ))]
    pub id: String,
    #[validate(length(
        min = 3,
        max = 255,
        message = "fails validation - must be 3-255 characters long"
    ))]
    pub title: String,
    #[validate(length(
        min = 3,
        max = 255,
        message = "fails validation - must be 3-255 characters long"
    ))]
    pub description: String,
    pub image_url: String,
    pub qty: i32,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct AssetResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
}

impl From<Asset> for AssetResponse {
    fn from(asset: Asset) -> Self {
        AssetResponse {
            id: asset.id,
            title: asset.title,
            description: asset.description,
            image_url: asset.image_url,
        }
    }
}
