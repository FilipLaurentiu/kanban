use std::io::Write;
use diesel::deserialize::FromSql;
use diesel::FromSqlRow;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use crate::schema::*;

#[derive(serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, FromSqlRow)]
#[serde(rename_all = "camelCase")]
#[diesel(sql_type = sql_types::StatusEnum)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

impl ToSql<sql_types::StatusEnum, Pg> for Status {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            Status::Todo => { out.write_all(b"todo")? }
            Status::Doing => { out.write_all(b"doing")? }
            Status::Done => { out.write_all(b"done")? }
        }

        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::StatusEnum, Pg> for Status {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"todo" => Ok(Status::Todo),
            b"doing" => Ok(Status::Doing),
            b"done" => Ok(Status::Done),
            _ => Err("Unrecognized enum variant".into())
        }
    }
}


#[derive(Default, serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}

#[derive(diesel::QueryableByName)]
pub struct StatusCount {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
    #[diesel(sql_type = sql_types::StatusEnum)]
    pub status: Status,
}

impl From<Vec<StatusCount>> for BoardSummary {
    fn from(counts: Vec<StatusCount>) -> Self {
        let mut summary = BoardSummary::default();
        for StatusCount { count, status } in counts {
            match status {
                Status::Todo => summary.todo += count,
                Status::Doing => summary.doing += count,
                Status::Done => summary.done += count
            }
        }
        summary
    }
}

// for POST requests

#[derive(serde::Deserialize, diesel::Insertable)]
#[diesel(table_name = boards)]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

// for PATCH requests
#[derive(serde::Deserialize, diesel::AsChangeset)]
#[diesel(table_name = cards)]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}
