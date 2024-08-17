use super::super::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::friend_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FriendRequest {
    pub id: uuid::Uuid,
    pub from: uuid::Uuid,
    pub to: uuid::Uuid,
    pub is_accepted: bool,
    pub sent_at: std::time::SystemTime,
    pub accepted_at: Option<std::time::SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::friend_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFriendRequest {
    pub from: uuid::Uuid,
    pub to: uuid::Uuid,
}

impl NewFriendRequest {
    pub fn new(from: uuid::Uuid, to: uuid::Uuid) -> Self {
        Self { from, to }
    }
    pub fn insert(&self, conn: &mut PgConnection) -> Result<FriendRequest, diesel::result::Error> {
        diesel::insert_into(schema::friend_requests::table)
           .values(self)
           .returning(FriendRequest::as_returning())
           .get_result(conn)
    }
}
