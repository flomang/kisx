use crate::{
    app::{users::UserResponse, AppState},
    models::{LotWithImages, LotStatus},
    utils::auth::authenticate_token,
};
use async_graphql::*;

use super::{
    articles::{
        comments::{CommentListResponse, GetComments},
        ArticleListResponse, ArticleResponse, ArticlesParams, FeedParams, GetArticle, GetArticles,
        GetFeed,
    },
    lots::{FilterLots, FilterLotsAuthenticated},
    profiles::{GetProfile, ProfileResponse},
    tags::{GetTags, TagsResponse},
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

    // get lots by authenticated user
    async fn get_user_lots<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: FilterLots,
    ) -> Result<Vec<LotWithImages>> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let user_id = auth.user.id;

        let res = state
            .db
            .send(FilterLotsAuthenticated { auth, params, owner_id: Some(user_id) })
            .await??;

        Ok(res)
    }

    // get lots for sale authenticated user
    async fn get_lots_for_sale<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: FilterLots,
    ) -> Result<Vec<LotWithImages>> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;

        let statuses = vec![LotStatus::ForSale.as_str().to_string()];
        let params = FilterLots {
            statuses,
            ..params
        };

        let res = state
            .db
            .send(FilterLotsAuthenticated { auth, params, owner_id: None })
            .await??;

        Ok(res)
    }
}
