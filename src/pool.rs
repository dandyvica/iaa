use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_pool<T: Into<u32>>(
    url: &str,
    nb_conn: T,
) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(nb_conn.into())
        .build(manager)
        .expect("Failed to create pool")
}
