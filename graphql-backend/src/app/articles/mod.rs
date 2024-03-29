pub mod comments;
use validator::Validate;

use crate::app::profiles::ProfileResponseInner;
use crate::utils::{
    auth::Auth,
    CustomDateTime,
};

// #[derive(Debug, Deserialize)]
// pub struct In<T> {
//     article: T,
// }

// Extractors ↓

#[derive(Debug, Deserialize)]
pub struct ArticlePath {
    pub slug: String,
}

#[derive(async_graphql::InputObject)]
#[derive(Debug, Deserialize)]
pub struct ArticlesParams {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<usize>,  // <- if not set, is 20
    pub offset: Option<usize>, // <- if not set, is 0
}

#[derive(async_graphql::InputObject)]
#[derive(Debug, Deserialize)]
pub struct FeedParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// Client Messages ↓

#[derive(async_graphql::InputObject)]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub tag_list: Vec<String>,
}

#[derive(Debug)]
pub struct CreateArticleOuter {
    pub auth: Auth,
    pub article: CreateArticle,
}

#[derive(Debug)]
pub struct GetArticle {
    pub auth: Option<Auth>,
    pub slug: String,
}

#[derive(async_graphql::InputObject)]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct UpdateArticleOuter {
    pub auth: Auth,
    pub slug: String,
    pub article: UpdateArticle,
}

#[derive(Debug)]
pub struct DeleteArticle {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct FavoriteArticle {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct UnfavoriteArticle {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct GetArticles {
    pub auth: Option<Auth>,
    pub params: ArticlesParams,
}

#[derive(Debug)]
pub struct GetFeed {
    pub auth: Auth,
    pub params: FeedParams,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct ArticleResponse {
    pub article: ArticleResponseInner,
}

#[derive(async_graphql::SimpleObject)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponseInner {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: ProfileResponseInner,
}

#[derive(async_graphql::SimpleObject)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleResponseInner>,
    pub articles_count: usize,
}
