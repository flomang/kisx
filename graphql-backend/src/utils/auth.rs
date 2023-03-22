use actix_web::{http::header::AUTHORIZATION, web::Data, HttpRequest};
use http::header::HeaderValue;

use crate::app::{AppState, Token};
use crate::models::User;
use crate::prelude::*;

const TOKEN_PREFIX: &str = "Token ";

// expand this as needed
#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub token: String,
}

// create auth message
#[derive(Debug)]
pub struct GenerateAuth {
    pub token: String,
}

// pub async fn authenticate(
//     state: &Data<AppState>,
//     req: &HttpRequest,
// ) -> Result<Auth, Error> {
//     let token  = preprocess_authz_token(req.headers().get(AUTHORIZATION))?;
//     let auth = state.db.send(GenerateAuth { token }).await??;
//     Ok(auth)
// }

// pub async fn authenticate_token(state: &AppState, token: String) -> Result<Auth, Error> {
//     let auth = state.db.send(GenerateAuth { token }).await??;
//     Ok(auth)
// }

pub async fn authenticate_token<'ctx>(
    state: &AppState,
    ctx: &async_graphql::Context<'ctx>,
) -> Result<Auth, Error> {
    let token = ctx.data::<Token>();
    match token {
        Ok(token) => {
            let token = token.0.clone();
            let auth = state.db.send(GenerateAuth { token }).await??;
            Ok(auth)
        }
        Err(_) => Err(Error::Unauthorized("no authorization was provided".to_string())),
    }
}

// fn preprocess_authz_token(token: Option<&HeaderValue>) -> Result<String> {
//     let token = match token {
//         Some(token) => token.to_str().unwrap(),
//         None => {
//             return Err(Error::Unauthorized(json!({
//                 "error": "No authorization was provided",
//             })))
//         }
//     };

//     if !token.starts_with(TOKEN_PREFIX) {
//         return Err(Error::Unauthorized(json!({
//             "error": "Invalid authorization method",
//         })));
//     }

//     let token = token.replacen(TOKEN_PREFIX, "", 1);

//     Ok(token)
// }
