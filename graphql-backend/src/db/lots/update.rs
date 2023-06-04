use super::DbExecutor;
use crate::models::{LotWithImages, LotImage};
use crate::{app::lots::UpdateLotAuthenticated, models::Lot, prelude::*};
use actix::prelude::*;
use diesel::prelude::*;


impl Message for UpdateLotAuthenticated {
    type Result = Result<LotWithImages>;
}

impl Handler<UpdateLotAuthenticated> for DbExecutor {
    type Result = Result<LotWithImages>;

    fn handle(&mut self, msg: UpdateLotAuthenticated, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lots::dsl::*;
        use crate::schema::lot_images::dsl::{lot_images, lot_id};

        let conn = &mut self.0.get()?;

        conn.transaction(|connection| {
            let updated: Lot = diesel::update(lots)
                .filter(user_id.eq(msg.auth.user.id))
                .filter(id.eq(msg.lot.id))
                .set(&msg.lot)
                .get_result(connection)?;

            // select all images for this lot
            let images: Vec<LotImage> = lot_images
                .filter(lot_id.eq(updated.id))
                .load(connection)
                .expect("Error loading posts");

            Ok(LotWithImages {
                lot: updated.into(),
                images,
            })
        })
    }
}
