use super::super::schema;
use diesel::{
    dsl::sql,
    prelude::*,
    sql_types::{Bool, Text},
    ExpressionMethods, QueryDsl,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
pub trait User {
    fn new(id: uuid::Uuid, name: String, email: String, password: String) -> Self;
}

#[derive(Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RawUser {
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

impl User for RawUser {
    fn new(id: uuid::Uuid, name: String, email: String, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
            image: None,
        }
    }
}

impl Queryable<users::SqlType, diesel::pg::Pg> for RawUser {
    type Row = (
        uuid::Uuid,
        String,
        String,
        String,
        Option<String>,
    );


    fn build(row: Self::Row) -> Self {
        Self {
            id: row.0,
            name: row.1,
            email: row.2,
            password: row.3,
            image: row.4,
        }
    }
}



#[derive(Default)]
pub struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    image: Option<String>,
}

impl UpdateUser {
    pub fn is_valid(&self) -> bool {
        self.name.is_some()
            || self.email.is_some()
            || self.password.is_some()
            || self.image.is_some()
    }
    pub fn new(
        name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        image: Option<String>,
    ) -> Self {
        Self {
            name,
            email,
            password,
            image,
        }
    }
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

    pub async fn insert(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(conn)
            .await
    }
}

impl User {
    pub async fn find_by_id(
        id: uuid::Uuid,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        users::table
            .find(id)
            .select(User::as_select())
            .first(conn)
            .await
            .optional()
    }

    pub async fn find_by_email(
        email: &str,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(conn)
            .await
            .optional()
    }

    pub async fn text_search(
        query: &str,
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        users::table
            .filter(
                sql::<Bool>(
                    "to_tsvector('english', name ||'' || email) @@ to_tsquery('english', '?')",
                )
                .bind::<Text, _>(query.to_owned()),
            )
            .select(User::as_select())
            .load(conn)
            .await
    }

    pub async fn add_image(
        &mut self,
        image: String,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self, diesel::result::Error> {
        Self::update(
            self.id,
            UpdateUser {
                image: Some(image),
                ..Default::default()
            },
            conn,
        )
        .await
    }

    pub async fn delete(
        id: uuid::Uuid,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self, diesel::result::Error> {
        diesel::delete(users::table.filter(users::id.eq(id)))
            .returning(User::as_returning())
            .get_result(conn)
            .await
    }

    pub async fn update(
        id: uuid::Uuid,
        update: UpdateUser,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self, diesel::result::Error> {
        if !update.is_valid() {
            return Err(diesel::result::Error::QueryBuilderError(
                "No update field provided".into(),
            ));
        }
        match Self::find_by_id(id, conn).await? {
            Some(user) => {
                let mut user = user;
                if let Some(name) = update.name {
                    user.name = name;
                }
                if let Some(email) = update.email {
                    user.email = email;
                }
                if let Some(password) = update.password {
                    user.password = password;
                }
                if let Some(image) = update.image {
                    user.image = Some(image);
                }
                diesel::update(users::table.filter(users::id.eq(id)))
                    .set(&user)
                    .returning(User::as_returning())
                    .get_result(conn)
                    .await
            }
            None => Err(diesel::result::Error::NotFound),
        }
    }
}
