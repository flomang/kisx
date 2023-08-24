use chrono::NaiveDateTime;

use crate::schema::prices;

#[derive(Debug, Queryable)]
pub struct Price {
    pub external_id: String,
    pub source: String,
    pub currency_symbol: String,
    pub recorded_at: NaiveDateTime,
    pub amount: f64,
}