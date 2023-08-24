use std::env;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2;

use crate::{schema, StdErr};
use crate::models::*;
use crate::schema::*;
use crate::schema::boards::dsl::boards;
use crate::schema::cards::dsl::cards;

type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;


pub struct Db {
    pool: PgPool,
}

impl Db {
    pub fn connect() -> Result<Self, StdErr> {
        let db_url = env::var("DATABASE_URL")?;
        let manager = r2d2::ConnectionManager::new(db_url);
        let pool = r2d2::Pool::new(manager)?;
        Ok(Self { pool })
    }

    pub fn boards(&self) -> Result<Vec<Board>, StdErr> {
        let mut conn = self.pool.get()?;
        let res: Vec<Board> = boards::table().load(&mut conn)?;
        Ok(res)
    }

    pub fn board_summary(&self, board_id: i64) -> Result<BoardSummary, StdErr> {
        let mut conn = self.pool.get()?;
        let counts: Vec<StatusCount> = diesel::sql_query(format!(
            "select count(*), status from cards where cards.board_id = {} group by status",
            board_id
        ))
            .load(&mut conn)?;
        Ok(counts.into())
    }


    pub fn create_board(&self, create_board: CreateBoard) -> Result<Board, StdErr> {
        let mut conn = self.pool.get()?;
        let board = diesel::insert_into(boards::table())
            .values(create_board)
            .get_result(&mut conn)?;
        Ok(board)
    }

    pub fn delete_board(&self, board_id: i64) -> Result<(), StdErr> {
        let mut conn = self.pool.get()?;
        diesel::delete(boards::table().filter(schema::boards::id.eq(board_id))).execute(&mut conn)?;
        Ok(())
    }

    pub fn cards(&self, board_id: i64) -> Result<Vec<Card>, StdErr> {
        let mut conn = self.pool.get()?;

        let res = cards::table()
            .filter(schema::cards::id.eq(&board_id))
            .load(&mut conn)?;
        Ok(res)
    }

    pub fn create_card(&self, create_card: CreateCard) -> Result<Card, StdErr> {
        let mut conn = self.pool.get()?;
        let card = diesel::insert_into(cards::table())
            .values(&create_card)
            .get_result(&mut conn)?;
        Ok(card)
    }

    pub fn update_card(&self, card_id: i64, update_card: UpdateCard) -> Result<Card, StdErr> {
        let mut conn = self.pool.get()?;
        let card = diesel::update(cards::table().filter(schema::cards::id.eq(card_id)))
            .set(update_card)
            .get_result(&mut conn)?;
        Ok(card)
    }

    pub fn delete_card(&self, card_id: i64) -> Result<(), StdErr> {
        let mut conn = self.pool.get()?;
        diesel::delete(
            cards::table().filter(schema::cards::id.eq(card_id)),
        )
            .execute(&mut conn)?;
        Ok(())
    }
}