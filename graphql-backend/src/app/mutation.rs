use async_graphql::*;
use validator::{Validate, ValidateArgs};
//use super::MyResult as Result;

use crate::{
    app::{
        users::{LoginUser, RegisterUser, UpdateUser, UpdateUserOuter, UserResponse},
        AppState,
    },
    error::validation_errors_to_error,
    utils::auth::authenticate_token,
};

use super::{
    articles::{
        comments::{AddComment, AddCommentOuter, CommentResponse, DeleteComment},
        ArticleResponse, CreateArticle, CreateArticleOuter, DeleteArticle, FavoriteArticle,
        UnfavoriteArticle, UpdateArticle, UpdateArticleOuter,
    },
    profiles::{FollowProfile, ProfileResponse, UnfollowProfile}, users::ForgotPassword, lots::{CreateLot, LotResponse, CreateLotOuter},
};
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // register a new user
    async fn signup<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: RegisterUser,
    ) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();

        params
            .validate_args((state, state))
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let res = state.db.send(params).await??;
        Ok(res)
    }

    // forgot password 
    async fn forgot_password<'ctx>(&self, ctx: &Context<'ctx>, params: ForgotPassword) -> Result<bool> {
        let state = ctx.data_unchecked::<AppState>();

        params
            .validate_args(state)
            .map_err(|e| validation_errors_to_error(e).extend())?;

        // TODO send email with reset link
        Ok(true)
    }

    // login a user
    async fn signin<'ctx>(&self, ctx: &Context<'ctx>, params: LoginUser) -> Result<UserResponse> {
        params
            .validate()
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let state = ctx.data_unchecked::<AppState>();
        let res = state.db.send(params).await??;
        Ok(res)
    }

    // update a user
    async fn update_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: UpdateUser,
    ) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;

        params
            .validate_args(state)
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let res = state
            .db
            .send(UpdateUserOuter {
                auth,
                update_user: params,
            })
            .await??;
        Ok(res)
    }

    // follow a user
    async fn follow_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state.db.send(FollowProfile { auth, username }).await??;
        Ok(res)
    }

    // unfollow a user
    async fn unfollow_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state.db.send(UnfollowProfile { auth, username }).await??;
        Ok(res)
    }

    // create article
    async fn create_acticle<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: CreateArticle,
    ) -> Result<ArticleResponse> {
        params
            .validate()
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state
            .db
            .send(CreateArticleOuter {
                auth,
                article: params,
            })
            .await??;

        Ok(res)
    }

    // create lot
    async fn create_lot<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: CreateLot,
    ) -> Result<LotResponse> {
        params
            .validate()
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state
            .db
            .send(CreateLotOuter {
                auth,
                lot: params,
            })
            .await??;

        Ok(res)
    }

    // update article
    async fn update_acticle<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
        params: UpdateArticle,
    ) -> Result<ArticleResponse> {
        params
            .validate()
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state
            .db
            .send(UpdateArticleOuter {
                auth,
                slug,
                article: params,
            })
            .await??;

        Ok(res)
    }

    // update article
    async fn delete_acticle<'ctx>(&self, ctx: &Context<'ctx>, slug: String) -> Result<bool> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        state.db.send(DeleteArticle { auth, slug }).await??;
        Ok(true)
    }

    // favorite article
    async fn favorite_acticle<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<ArticleResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state.db.send(FavoriteArticle { auth, slug }).await??;
        Ok(res)
    }

    // unfavorite article
    async fn unfavorite_acticle<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<ArticleResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state.db.send(UnfavoriteArticle { auth, slug }).await??;
        Ok(res)
    }

    // add comment to article
    async fn add_comment<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
        comment: AddComment,
    ) -> Result<CommentResponse> {
        comment
            .validate()
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        let res = state
            .db
            .send(AddCommentOuter {
                auth,
                slug,
                comment,
            })
            .await??;
        Ok(res)
    }

    // delete comment from article
    async fn delete_comment<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
        comment_id: i32,
    ) -> Result<bool> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        state
            .db
            .send(DeleteComment {
                auth,
                slug,
                comment_id,
            })
            .await??;
        Ok(true)
    }
}
