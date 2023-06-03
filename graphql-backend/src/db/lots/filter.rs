use super::DbExecutor;
use crate::{
    app::lots::FilterLotsAuthenticated,
    models::{Lot, LotImage, LotWithImages},
    prelude::*,
};
use actix::prelude::*;
use diesel::prelude::*;


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
