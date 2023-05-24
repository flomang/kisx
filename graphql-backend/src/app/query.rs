use crate::{
    app::{users::UserResponse, AppState},
    utils::auth::authenticate_token, models::LotWithImages,
};
use async_graphql::*;

use super::{
    articles::{
        comments::{CommentListResponse, GetComments},
        ArticleListResponse, ArticleResponse, ArticlesParams, FeedParams, GetArticle, GetArticles,
        GetFeed,
    },
    profiles::{GetProfile, ProfileResponse},
    tags::{GetTags, TagsResponse}, lots::{FilterLots, FilterLotsAuthenticated},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // get the current logged in user by token
    async fn get_current_user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        Ok(UserResponse::create_with_auth(auth))
    }

    // get profile for username
    async fn get_profile<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx)
            .await
            .map(|auth| Some(auth))
            .unwrap_or(None);

        let res = state.db.send(GetProfile { auth, username }).await??;

        Ok(res)
    }

    // get article by slug
    async fn get_article<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<ArticleResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx)
            .await
            .map(|auth| Some(auth))
            .unwrap_or(None);

        let res = state.db.send(GetArticle { auth, slug }).await??;

        Ok(res)
    }

    // get articles with optional filter params
    async fn get_articles<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        filter: ArticlesParams,
    ) -> Result<ArticleListResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx)
            .await
            .map(|auth| Some(auth))
            .unwrap_or(None);

        let res = state
            .db
            .send(GetArticles {
                auth,
                params: filter,
            })
            .await??;

        Ok(res)
    }

    // get articles with optional filter params
    async fn get_article_feed<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: FeedParams,
    ) -> Result<ArticleListResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;

        let res = state.db.send(GetFeed { auth, params }).await??;

        Ok(res)
    }

    // get comments for article
    async fn get_comments<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<CommentListResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx)
            .await
            .map(|auth| Some(auth))
            .unwrap_or(None);

        let res = state.db.send(GetComments { auth, slug }).await??;

        Ok(res)
    }

    // get tags
    async fn get_tags<'ctx>(&self, ctx: &Context<'ctx>) -> Result<TagsResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let res = state.db.send(GetTags {}).await??;

        Ok(res)
    }

    // get lots
    async fn get_lots<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: FilterLots,
    ) -> Result<Vec<LotWithImages>> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state
            .db
            .send(FilterLotsAuthenticated { auth, params })
            .await??;

        Ok(res)
    }
}
