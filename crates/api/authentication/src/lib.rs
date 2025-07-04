pub use crate::models::user::RegisterUser;
pub mod handlers;
pub mod models;
mod config;

pub use diesel::r2d2::{ConnectionManager, Pool};
pub use diesel::PgConnection;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
