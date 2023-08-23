mod logger;
mod models;
mod schema;
mod db;


#[macro_use] extern crate diesel;
type StdErr = Box<dyn std::error::Error>;


fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;

    logger::init()?;
    Ok(())
}
