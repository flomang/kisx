use crate::{
    models::{Lot, LotImage, NewLot, NewLotImage},
    prelude::*,
};
use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::lots::{CreateLotOuter, LotResponse};

impl Message for CreateLotOuter {
    type Result = Result<LotResponse>;
}

// #[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CreateLot {
//     #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
//     pub category: String,
//     #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
//     pub condition: String,
//     #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
//     pub tag: String,
//     #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
//     pub description: String,
//     // pub images: serde_json::Value,
//     pub images: Vec<CreateLotImage>,
//     pub data: serde_json::Value,
// }

// #[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CreateLotImage {
//     pub image_url: String,
//     pub is_thumbnail: bool,
// }

// impl Handler<CreateLotOuter> for DbExecutor {
//     type Result = Result<LotResponse>;

//     fn handle(&mut self, msg: CreateLotOuter, _: &mut Self::Context) -> Self::Result {
//         use crate::schema::lots::dsl::*;

//         let new_lot = NewLot {
//             user_id: msg.auth.user.id,
//             category: msg.lot.category.clone(),
//             condition: msg.lot.condition.clone(),
//             tag: Some(msg.lot.tag),
//             description: Some(msg.lot.description),
//             meta_data: serde_json::to_value(msg.lot.data).unwrap(),
//         };

//         let conn = &mut self.0.get()?;

//         match diesel::insert_into(lots)
//             .values(new_lot)
//             .get_result::<Lot>(conn)
//         {
//             Ok(lot) => Ok(lot.into()),
//             Err(e) => Err(e.into()),
//         }
//     }
// }

impl Handler<CreateLotOuter> for DbExecutor {
    type Result = Result<LotResponse>;

    fn handle(&mut self, msg: CreateLotOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{lot_images::dsl::*, lots::dsl::*};

        let new_lot = NewLot {
            user_id: msg.auth.user.id,
            category: msg.lot.category.clone(),
            condition: msg.lot.condition.clone(),
            tag: Some(msg.lot.tag),
            description: Some(msg.lot.description),
            meta_data: serde_json::to_value(msg.lot.meta_data).unwrap(),
        };

        let conn = &mut self.0.get()?;

        conn.transaction(|connection| {
            let inserted_lot: Lot = diesel::insert_into(lots)
                .values(new_lot)
                .get_result(connection)?;

            let new_lot_images: Vec<NewLotImage> = msg
                .lot
                .images
                .into_iter()
                .map(|image| NewLotImage {
                    lot_id: inserted_lot.id,
                    image_url: image.image_url,
                    is_thumbnail: image.is_thumbnail,
                })
                .collect();

            let inserted_images: Vec<LotImage> = diesel::insert_into(lot_images)
                .values(&new_lot_images)
                .get_results(connection)?;

            Ok(LotResponse {
                lot: inserted_lot.into(),
                images: inserted_images.into_iter().map(|image| image.into()).collect(),
            })
        })
    }
}
