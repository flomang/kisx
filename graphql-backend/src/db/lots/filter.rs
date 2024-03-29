use std::str::FromStr;

use super::DbExecutor;
use crate::{
    app::lots::FilterLotsAuthenticated,
    models::{Lot, LotImage, LotWithImages, LotStatus},
    prelude::*,
};
use actix::prelude::*;
use diesel::prelude::*;
use diesel::sql_types::Text;
sql_function!(fn lower(x: Text) -> Text);

// Messages
// Filter lots by authenticated user
impl Message for FilterLotsAuthenticated {
    type Result = Result<Vec<LotWithImages>>;
}

// Handlers
impl Handler<FilterLotsAuthenticated> for DbExecutor {
    type Result = Result<Vec<LotWithImages>>;

    fn handle(&mut self, msg: FilterLotsAuthenticated, _: &mut Self::Context) -> Self::Result {
        use crate::schema::lots::dsl::*;

        let conn = &mut self.0.get()?;

        let mut user_lots_query = if let Some(user) = msg.owner_id {
            lots.filter(user_id.eq(user)).into_boxed()
        } else {
            lots.into_boxed()
        };

        // remove soft deleted lots
        user_lots_query = user_lots_query.filter(status.ne(LotStatus::Deleted.as_str()));

        if !msg.params.statuses.is_empty() {
            user_lots_query = user_lots_query.filter(status.eq_any(&msg.params.statuses));
        }
        if !msg.params.categories.is_empty() {
            user_lots_query = user_lots_query.filter(category.eq_any(&msg.params.categories));
        }
        if !msg.params.conditions.is_empty() {
            user_lots_query = user_lots_query.filter(condition.eq_any(&msg.params.conditions));
        }
        if !msg.params.terms.is_empty() {
            let ilike_terms: Vec<_> = msg
                .params
                .terms
                .iter()
                .map(|term| format!("%{}%", term.to_lowercase()))
                .collect();

            let ids: Vec<_> = msg
                .params
                .terms
                .iter()
                .map(|term| term.to_lowercase())
                .collect();

            // lower the description column and check if it contains any of the lowercase terms
            // lower the title column and check if it contains any of the lowercase terms
            // check if the external_id column is equal to any of the lowercase terms
            user_lots_query = user_lots_query.filter(
                lower(description)
                    .like(ilike_terms.join(""))
                    .or(lower(title).like(ilike_terms.join("")))
                    .or(external_id.eq_any(ids)),
            );
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

