use super::DbExecutor;
use crate::app::lots::{CreateLotAuthenticated};
use crate::models::LotWithImages;
use crate::{
    models::{Lot, LotImage, NewLot, NewLotImage},
    prelude::*,
};
use actix::prelude::*;
use diesel::prelude::*;


impl Message for CreateLotAuthenticated {
    type Result = Result<LotWithImages>;
}

impl Handler<CreateLotAuthenticated> for DbExecutor {
    type Result = Result<LotWithImages>;

    fn handle(&mut self, msg: CreateLotAuthenticated, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{lot_images::dsl::*, lots::dsl::*};

        let new_lot = NewLot {
            user_id: msg.auth.user.id,
            category: msg.lot.category.clone(),
            condition: msg.lot.condition.clone(),
            title: msg.lot.title.clone(),
            external_id: msg.lot.external_id.clone(),
            description: msg.lot.description,
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

            Ok(LotWithImages {
                lot: inserted_lot,
                images: inserted_images
                    .into_iter()
                    .map(|image| image.into())
                    .collect(),
            })
        })
    }
}
