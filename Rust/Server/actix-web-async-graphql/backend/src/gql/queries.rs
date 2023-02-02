/// 查询总线
use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::users::models::User;
use crate::users::services::{all_users, get_user_by_email};
use crate::util::constant::GqlResult;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // 获取全部用户,
    async fn all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        all_users(my_pool).await
    }

    // 根据 email获取用户
    async fn get_user_by_email(&self, ctx: &Context<'_>, email: String) -> GqlResult<User> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        get_user_by_email(my_pool, &email).await
    }
}
