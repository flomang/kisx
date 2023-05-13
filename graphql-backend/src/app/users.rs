use regex::Regex;
use std::convert::From;
use validator::{Validate, ValidationError};

use crate::models::User;
use crate::utils::{auth::Auth, jwt::CanGenerateJwt};

use super::AppState;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]+$").unwrap();
}

// #[derive(Debug, Deserialize)]
// pub struct In<U> {
//     user: U,
// }

// Client Messages ↓
#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(
        length(min = 3, message = "must be at least 3 characters"),
        custom(
            function = "validate_unique_username",
            arg = "&'v_a AppState",
            message = "username already taken"
        )
    )]
    pub username: String,
    #[validate(
        email(message = "not a valid email address"),
        custom(
            function = "validate_unique_email",
            arg = "&'v_a AppState",
            message = "email already registered"
        )
    )]
    pub email: String,
    #[validate(
        length(min = 8, max = 72, message = "must be 8-72 characters"),
        custom(
            function = "validate_password",
            message = "password must contain at least one uppercase letter, one lowercase letter, one number, one special character, and be at least 8 characters long"
        )
    )]
    #[graphql(secret)]
    pub password: String,
}

fn validate_unique_username(username: &str, state: &AppState) -> Result<(), ValidationError> {
    let result = async_std::task::block_on(state.db.send(FindUser {
        username: username.trim().to_string(),
    }))
    .unwrap();

    match result {
        // if the username is already taken, return an error
        Ok(_) => Err(ValidationError::new("invalid_username")),
        Err(_) => Ok(()),
    }
}

fn validate_unique_email(email: &str, state: &AppState) -> Result<(), ValidationError> {
    let result = async_std::task::block_on(state.db.send(FindEmail {
        email: email.trim().to_string(),
    }))
    .unwrap();

    match result {
        // if the username is already taken, return an error
        Ok(_) => Err(ValidationError::new("invalid_username")),
        Err(_) => Ok(()),
    }
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    // and special characters
    if password.chars().any(char::is_uppercase)
        && password.chars().any(char::is_lowercase)
        && password.chars().any(char::is_numeric)
        && password.chars().any(|c| !c.is_alphanumeric())
    {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_password"))
    }
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email(message = "not a valid email address"))]
    pub email: String,
    pub password: String,
}

pub struct FindUser {
    pub username: String,
}

pub struct FindEmail {
    pub email: String,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(
        length(min = 3, message = "must be at least 3 characters long"),
        custom(
            function = "validate_unique_username",
            arg = "&'v_a AppState",
            message = "already taken"
        )
    )]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8, max = 72, message = "must be 8-72 characters long"))]
    pub password: Option<String>,
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub bio: Option<String>,
    #[validate(url(message = "is not a URL"))]
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct UpdateUserOuter {
    pub auth: Auth,
    pub update_user: UpdateUser,
}

// JSON response objects ↓

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct UserResponse {
    pub user: UserResponseInner,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: user.generate_jwt().unwrap(),
                email: user.email,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

impl UserResponse {
    pub fn create_with_auth(auth: Auth) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: auth.token,
                email: auth.user.email,
                username: auth.user.username,
                bio: auth.user.bio,
                image: auth.user.image,
            },
        }
    }
}
