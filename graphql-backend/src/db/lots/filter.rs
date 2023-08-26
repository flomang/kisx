use std::str::FromStr;

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

use diesel::sql_types::Text;
sql_function!(fn lower(x: Text) -> Text);

impl Handler<FilterLotsAuthenticated> for DbExecutor {
    type Result = Result<Vec<LotWithImages>>;

    fn handle(&mut self, msg: FilterLotsAuthenticated, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lots::dsl::*;

        let conn = &mut self.0.get()?;

        let mut user_lots_query = lots.filter(user_id.eq(msg.auth.user.id)).into_boxed();

        if !msg.params.statuses.is_empty() {
            user_lots_query = user_lots_query.filter(status.eq_any(msg.params.statuses));
        }
        if !msg.params.categories.is_empty() {
            user_lots_query = user_lots_query.filter(category.eq_any(msg.params.categories));
        }
        if !msg.params.conditions.is_empty() {
            user_lots_query = user_lots_query.filter(condition.eq_any(msg.params.conditions));
        }
        if !msg.params.terms.is_empty() {
            let ilike_conditions: Vec<_> = msg
                .params
                .terms
                .iter()
                .map(|term| format!("%{}%", term.to_lowercase()))
                .collect();

            // lower the description column and check if it contains any of the lowercase terms
            user_lots_query = user_lots_query.filter(
                lower(description)
                    .like(ilike_conditions.join(""))
                    .or(lower(title).like(ilike_conditions.join(""))),
            );
            // .filter(lower(title).like(ilike_conditions.join("")));
        }

        let user_lots = user_lots_query.select(Lot::as_select()).load(conn)?;

        // lot images
        let images = LotImage::belonging_to(&user_lots)
            .select(LotImage::as_select())
            .load(conn)?;

        // group the lots with images
        let lots_with_images = images
            .grouped_by(&user_lots)
            .into_iter()
            .zip(user_lots)
            .map(|(imgs, lot)| LotWithImages { lot, images: imgs })
            .collect::<Vec<LotWithImages>>();

        Ok(lots_with_images)
    }
}
