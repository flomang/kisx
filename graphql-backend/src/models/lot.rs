use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{lots::{self}, lot_images};

#[derive(async_graphql::SimpleObject, Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.

pub struct Lot {
    #[graphql(skip)]
    pub id: Uuid,
    #[graphql(skip)]
    pub user_id: Uuid,
    pub category: String,
    pub condition: String,
    pub title: String,
    pub external_id: Option<String>,
    pub description: String,
    pub meta_data: Option<serde_json::Value>,
    #[graphql(skip)]
    pub created_at: NaiveDateTime,
    #[graphql(skip)]
    pub updated_at: NaiveDateTime,
    pub status: String,
}

#[async_graphql::ComplexObject]
impl Lot {
    async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn user_id(&self) -> String {
        self.user_id.to_string()
    }
    async fn created_at(&self) -> String {
        self.created_at.to_string()
    }
    async fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}

#[derive(async_graphql::SimpleObject, Debug, Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.
#[diesel(belongs_to(Lot))]
pub struct LotImage {
    #[graphql(skip)]
    pub id: Uuid,
    #[graphql(skip)]
    pub lot_id: Uuid,
    pub image_url: String,
    pub is_thumbnail: bool,
    #[graphql(skip)]
    pub created_at: NaiveDateTime,
    #[graphql(skip)]
    pub updated_at: NaiveDateTime,
}

#[async_graphql::ComplexObject]
impl LotImage {
    async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn lot_id(&self) -> String {
        self.lot_id.to_string()
    }
    async fn created_at(&self) -> String {
        self.created_at.to_string()
    }
    async fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Debug)]
pub struct LotWithImages {
    #[serde(flatten)]
    pub lot: Lot,
    pub images: Vec<LotImage>,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = lots)]
pub struct NewLot {
    pub user_id: Uuid,
    pub category: String,
    pub condition: String,
    pub title: String,
    pub external_id: Option<String>,
    pub description: String,
    pub meta_data: serde_json::Value,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = lot_images)]
pub struct NewLotImage {
    pub lot_id: Uuid,
    pub image_url: String,
    pub is_thumbnail: bool,
}

#[derive(Debug, Identifiable, AsChangeset)]
#[diesel(table_name = lots)]
pub struct UpdateLot {
    pub id: Uuid,
    pub category: Option<String>,
    pub condition: Option<String>,
    pub title: Option<String>,
    pub external_id: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

pub enum LotStatus {
    Cancelled,
    Deleted,
    Drafted,
    ForSale,
    Pending,
    Sold,
    Archived,
}

impl LotStatus {
    pub fn as_str(&self) -> &str {
        match self {
            LotStatus::Cancelled => "cancelled sale",
            LotStatus::Deleted => "deleted",
            LotStatus::Drafted => "drafted",
            LotStatus::ForSale => "for sale",
            LotStatus::Pending => "pending sale",
            LotStatus::Sold => "sold",
            LotStatus::Archived => "archived",
        }
    }
}
