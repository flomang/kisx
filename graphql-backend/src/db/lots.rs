
use actix::prelude::*;
use diesel::prelude::*;
use crate::{prelude::*, models::{NewLot, Lot}};

use super::DbExecutor;
use crate::app::lots::{CreateLotOuter, LotResponse};

impl Message for CreateLotOuter {
    type Result = Result<LotResponse>;
}

impl Handler<CreateLotOuter> for DbExecutor {
    type Result = Result<LotResponse>;

    fn handle(&mut self, msg: CreateLotOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lots::dsl::*;

        let new_lot = NewLot {
            user_id: msg.auth.user.id,
            category: msg.lot.category.clone(),
            condition: msg.lot.condition.clone(),
            tag: Some(msg.lot.tag),
            description: Some(msg.lot.description),
            images: serde_json::to_value(msg.lot.images).unwrap(),
            data: serde_json::to_value(msg.lot.data).unwrap(),
        };

        let conn = &mut self.0.get()?;

        match diesel::insert_into(lots)
            .values(new_lot)
            .get_result::<Lot>(conn)
        {
            Ok(lot) => Ok(lot.into()),
            Err(e) => Err(e.into()),
        }
    }
}