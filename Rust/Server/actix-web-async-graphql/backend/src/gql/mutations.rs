// 更变总线（新增，修改）
use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::users::models::{NewUser, User};
use crate::users::services::new_user;
use crate::util::constant::GqlResult;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // 新增用户
    async fn new_user(&self, ctx: &Context<'_>, user: NewUser) -> GqlResult<User> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        new_user(my_pool, user).await
    }
}
