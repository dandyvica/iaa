use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_pool(url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Failed to create pool")
}
