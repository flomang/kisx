use super::DbExecutor;
use crate::{app::lots::DeleteLotAuthenticated, prelude::*};
use actix::prelude::*;
use diesel::prelude::*;


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
