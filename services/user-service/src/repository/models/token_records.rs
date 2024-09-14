use diesel::{
    dsl::sql,
    prelude::*,
    sql_types::{Bool, Text},
    ExpressionMethods, QueryDsl,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::repository::schema::{self, sql_types::InvalidTokenReasonEnum};
#[derive(Selectable, Identifiable, Queryable, Insertable)]
#[diesel(table_name = schema::invalid_token_records)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenRecords {
    pub id: uuid::Uuid,
    pub token_value: String,
    pub invalid_reason: InvalidTokenReasonEnum,
}

impl TokenRecords {
    pub async fn insert_new_token_record(
        self,
        conn: &mut AsyncPgConnection,
    ) -> Result<TokenRecords, diesel::result::Error> {
        diesel::insert_into(schema::invalid_token_records::table)
           .values(self)
           .get_result(conn)
           .await
    }

    pub async fn check_token_exists_by_its_value(
        token_value: String,
        conn: &mut AsyncPgConnection,
    ) -> Result<bool, diesel::result::Error> {
        let result = schema::invalid_token_records::table
           .filter(schema::invalid_token_records::token_value.eq(token_value))
           .select(Self::as_select())
           .first(conn)
           .await
           .optional();
        result.map(|r| r.is_some())
    }
}