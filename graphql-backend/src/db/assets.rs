use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::assets::{AddAssetRequest, AssetResponse};
use crate::models::{Asset, NewAsset};
use crate::prelude::*;


impl Message for AddAssetRequest {
    type Result = Result<AssetResponse>;
}

impl Handler<AddAssetRequest> for DbExecutor {
    type Result = Result<AssetResponse>;

    fn handle(&mut self, msg: AddAssetRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::assets::dsl::*;

        let new_asset = NewAsset {
            id: msg.id.clone(),
            image_url: msg.image_url.clone(),
            title: msg.title.clone(),
            description: msg.description.clone(),
        }; 

        let conn = &mut self.0.get()?;

        match diesel::insert_into(assets)
            .values(new_asset)
            .get_result::<Asset>(conn)
        {
            Ok(o) => Ok(o.into()),
            Err(e) => {
                // TODO better handling of errors
                println!("Error: {:?}", e);
                Err(e.into())}
                ,
        }
    }
}