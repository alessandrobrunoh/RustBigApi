use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub mod users_repo;
