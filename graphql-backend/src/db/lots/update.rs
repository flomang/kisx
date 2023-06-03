use super::DbExecutor;
use crate::app::lots::LotResponse;
use crate::{app::lots::UpdateLotAuthenticated, models::Lot, prelude::*};
use actix::prelude::*;
use diesel::prelude::*;


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
                .filter(id.eq(msg.lot.id))
                .set(&msg.lot)
                .get_result(connection)?;

            Ok(LotResponse {
                lot: updated.into(),
                images: vec![],
            })
        })
    }
}
