use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = std::env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Failed to create pool")
}