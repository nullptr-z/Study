use serde::{Deserialize, Serialize};

// 关联 users表
#[rbatis::crud_enable(table_name:"user")]
// async_graphql::SimpleObject 简单对象，省去 getter、setter：
// 自动处理一些类型转换，例如：String -> str
#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)] // 将复杂对象类型和简单对象类型整合使用，ComplexObject
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub cred: String,
}

// #[async_graphq2l::Object]
#[async_graphql::ComplexObject] //将复杂对象类型和简单对象类型整合使用
impl User {
    //#region   复杂类型
    pub async fn from(&self) -> String {
        let mut from = String::new();
        from.push_str(&self.username);
        from.push_str("<");
        from.push_str(&self.email);
        from.push_str(">");
        // let from = format!("{}<{}>", &self.username, &self.email);
        from
    }
    //#endregion

    //#region   简单类型
    /* 使用 async_graphql::SimpleObject省略一下简单对象的定义 */
    // pub async fn id(&self) -> i32 {
    //     self.id
    // }
    // pub async fn email(&self) -> &str {
    //     self.email.as_str()
    // }
    // pub async fn username(&self) -> &str {
    //     self.username.as_str()
    // }
    //#endregion
}

#[rbatis::crud_enable(table_name:"user")]
#[derive(async_graphql::InputObject, Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    #[graphql(skip)] // 其表示此字段不会映射到 GraphQL
    pub id: i32,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub cred: String,
}
