//  注:
//  actix-web 的请求处理函数中，请求为 HttpRequest 类型，响应类型则是 HttpResponse。
//  而 async-graphql 在执行 GraphQL 服务时，请求类型和返回类型与 actix-web 的并不同，需要进行封装处理:
//  use async_graphql_actix_web::{Request, Response}

pub mod mutations;
pub mod queries;

use actix_web::{web, HttpResponse, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription,
    Schema, // EmptyMutation,
};
use async_graphql_actix_web::{Request, Response};

use crate::dbs::mysql::my_pool;
use crate::gql::{mutations::MutationRoot, queries::QueryRoot};
use crate::CFG;
// use crate::gql::queries::QueryRoot;

// `ActixSchema` 类型定义，项目中可以统一放置在一个共用文件中。
// 但 `actix-web` 和 `tide` 框架不同，无需放入应用程序`状态（State）`
// 所以此 `Schema` 类型仅是为了代码清晰易读，使用位置并不多，我们直接和构建函数一起定义。
// 或者，不做此类型定义，直接作为构建函数的返回类型。
type ActixSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn build_schema() -> ActixSchema {
    // 获取 mysql 数据池后，可以将其增加到：
    // 1. 作为 async-graphql 的全局数据；
    // 2. 作为 actix-web 的应用程序数据，优势是可以进行原子操作；
    // 3. 使用 lazy-static.rs
    let my_pool = my_pool().await;

    // MutationRoot：  EmptyMutation好像是默认更变操作，在没有自定义的更变是使用
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(my_pool)
        .finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
    let resp = schema.execute(req.into_inner()).await.into();
    resp
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(CFG.get("GQL_VER").unwrap())
                .subscription_endpoint(CFG.get("GQL_VER").unwrap()),
        )))
}
