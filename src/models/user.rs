use crate::schema::user;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
// user，为schema对象，而非数据库表名，所以需使用use crate::schema::user;引入相关item，否则会报找不到模块或类型错误
#[table_name="user"]  
pub struct CreateUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub nickname: &'a str,
    pub phone: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

