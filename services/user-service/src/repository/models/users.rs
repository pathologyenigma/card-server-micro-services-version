use diesel::{dsl::sql, prelude::*, r2d2::R2D2Connection, sql_types::{Bool, Text}};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use super::super::schema;
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub image: Option<String>,
}

use schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    id: uuid::Uuid,
    name: String,
    email: String,
    password: String,
}

impl NewUser {
    pub fn new(id: uuid::Uuid, name: String, email: String, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
        }
    }

    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
           .values(self)
           .returning(User::as_returning())
           .get_result(conn).await
    }
}

impl User {
    pub async fn find_by_id(id: uuid::Uuid, conn: &mut AsyncPgConnection) -> Result<Option<Self>, diesel::result::Error> {
        users::table.find(id)
        .select(User::as_select())
        .first(conn).await
        .optional()
    }

    pub async fn find_by_email(email: &str, conn: &mut AsyncPgConnection) -> Result<Option<Self>, diesel::result::Error> {
        users::table.filter(users::email.eq(email))
        .select(User::as_select())
        .first(conn).await
        .optional()
    }

    pub async fn text_search(query: &str, conn: &mut AsyncPgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        users::table.filter(sql::<Bool>("to_tsvector('english', name ||'' || email) @@ to_tsquery('english', '?')").bind::<Text,_>(query.to_owned()))
        .select(User::as_select())
        .load(conn).await
    }

    pub async fn add_image(&mut self, image: String, conn: &mut AsyncPgConnection) -> Result<Self, diesel::result::Error> {
        diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
        .set(users::image.eq(Some(image)))
        .returning(User::as_returning())
        .get_result(conn).await
    }

    pub async fn delete_user(id: uuid::Uuid, conn: &mut AsyncPgConnection) -> Result<Self, diesel::result::Error> {
        diesel::delete(users::table.filter(users::id.eq(id)))
        .returning(User::as_returning())
        .get_result(conn).await
    }

    pub async fn update_user(&mut self, conn: &mut AsyncPgConnection) -> Result<Self, diesel::result::Error> {
        diesel::update(users::table.filter(users::id.eq(self.id)))
        .set(self)
        .returning(User::as_returning())
        .get_result(conn).await
    }
}