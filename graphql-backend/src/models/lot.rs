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
    pub tag: Option<String>,
    pub description: Option<String>,
    pub meta_data: Option<serde_json::Value>,
    #[graphql(skip)]
    pub created_at: NaiveDateTime,
    #[graphql(skip)]
    pub updated_at: NaiveDateTime,
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

#[derive(Debug, Queryable, Identifiable, Associations, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(Lot))]
pub struct LotImage {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub image_url: String,
    pub is_thumbnail: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[async_graphql::Object]
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
    async fn image_url(&self) -> String {
        self.image_url.to_string()
    }
    async fn is_thumbnail(&self) -> bool {
        self.is_thumbnail
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