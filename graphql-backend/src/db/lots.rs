use crate::{
    app::lots::{DeleteLotAuthenticated, FilterLotsAuthenticated, UpdateLotAuthenticated},
    models::{Lot, LotImage, LotWithImages, NewLot, NewLotImage},
    prelude::*,
};
use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::lots::{CreateLotAuthenticated, LotResponse};

impl Message for CreateLotAuthenticated {
    type Result = Result<LotResponse>;
}

impl Handler<CreateLotAuthenticated> for DbExecutor {
    type Result = Result<LotResponse>;

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

            Ok(LotResponse {
                lot: inserted_lot.into(),
                images: inserted_images
                    .into_iter()
                    .map(|image| image.into())
                    .collect(),
            })
        })
    }
}

impl Message for UpdateLotAuthenticated {
    type Result = Result<LotResponse>;
}

impl Handler<UpdateLotAuthenticated> for DbExecutor {
    type Result = Result<LotResponse>;

    fn handle(&mut self, msg: UpdateLotAuthenticated, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lots::dsl::*;

        let conn = &mut self.0.get()?;

        conn.transaction(|connection| {
            let updated: Lot = diesel::update(lots)
                .filter(user_id.eq(msg.auth.user.id))
                .set(&msg.lot)
                .get_result(connection)?;

            Ok(LotResponse {
                lot: updated.into(),
                images: vec![],
            })
        })
    }
}

impl Message for DeleteLotAuthenticated {
    type Result = Result<usize>;
}

impl Handler<DeleteLotAuthenticated> for DbExecutor {
    type Result = Result<usize>;

    fn handle(&mut self, msg: DeleteLotAuthenticated, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lots::dsl::*;

        // delete lot where user_id = msg.auth.user.id and lot_id = msg.lot.id
        let conn = &mut self.0.get()?;
        let deleted = diesel::delete(
            lots.filter(user_id.eq(msg.auth.user.id))
                .filter(id.eq(msg.lot_id)),
        )
        .execute(conn)?;
        Ok(deleted)
    }
}

impl Message for FilterLotsAuthenticated {
    type Result = Result<Vec<LotWithImages>>;
}

impl Handler<FilterLotsAuthenticated> for DbExecutor {
    type Result = Result<Vec<LotWithImages>>;

    fn handle(&mut self, msg: FilterLotsAuthenticated, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lots::dsl::*;

        let conn = &mut self.0.get()?;

        let user_lots = lots
            .filter(user_id.eq(msg.auth.user.id))
            .select(Lot::as_select())
            .load(conn)?;

        // lot images
        let images = LotImage::belonging_to(&user_lots)
            .select(LotImage::as_select())
            .load(conn)?;

        // group the pages per book
        let lots_with_images = images
            .grouped_by(&user_lots)
            .into_iter()
            .zip(user_lots)
            .map(|(imgs, lot)| LotWithImages { lot, images: imgs })
            .collect::<Vec<LotWithImages>>();

        Ok(lots_with_images)
    }
}
