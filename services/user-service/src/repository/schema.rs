// @generated automatically by Diesel CLI.

pub mod sql_types {
    use std::io::Write;

    use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::{ Pg, PgValue}, serialize::{IsNull, ToSql}};

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "invalid_token_reason"))]
    pub struct InvalidTokenReason;
    #[derive(Debug, FromSqlRow, AsExpression, PartialEq, Eq)]
    #[diesel(sql_type = InvalidTokenReason)]
    pub enum InvalidTokenReasonEnum {
        Blocked,
        AlreadyLogOut
    }

    impl ToSql<InvalidTokenReason, Pg> for InvalidTokenReasonEnum {
        fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b , '_, Pg>) -> diesel::serialize::Result {
            match *self {
                InvalidTokenReasonEnum::Blocked => out.write_all(b"BLOCKED")?,
                InvalidTokenReasonEnum::AlreadyLogOut => out.write_all(b"ALREADYLOG_OUT")?,
            }
            Ok(IsNull::No)
        }
    }
    impl FromSql<InvalidTokenReason, Pg> for InvalidTokenReasonEnum {
        fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
            match bytes.as_bytes() {
                b"BLOCKED" => Ok(InvalidTokenReasonEnum::Blocked),
                b"ALREADYLOG_OUT" => Ok(InvalidTokenReasonEnum::AlreadyLogOut),
                _ => Err("Unrecognized invalid_token_reason".into()),
            }
        }
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::InvalidTokenReason;

    invalid_token_records (id) {
        id -> Uuid,
        token_value -> Text,
        invalid_reason -> InvalidTokenReason,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 16]
        name -> Varchar,
        #[max_length = 56]
        email -> Varchar,
        password -> Varchar,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        description -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    invalid_token_records,
    users,
);
