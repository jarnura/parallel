pub struct Store<T>
where
    T: DatabasePooling,
{
    pub master_pool: T::ConnectionPool,
}

pub async fn get_connection<T: DatabasePooling>(pool: &T) -> T::PooledConnection<'_> {
    T::get_connection(pool).await
}

#[async_trait::async_trait]
pub trait DatabasePooling {
    type ConnectionManager;
    type ConnectionPool;
    type PooledConnection<'a>
    where
        Self: 'a;

    async fn build_pool(database_url: &str, max_size: u32) -> Self::ConnectionPool;
    async fn get_connection(&self) -> Self::PooledConnection<'_>;
}

#[cfg(feature = "diesel")]
pub struct DieselAsync {
    pool: bb8::Pool<async_bb8_diesel::ConnectionManager<diesel::PgConnection>>,
}

#[cfg(feature = "diesel")]
impl DieselAsync {
    pub async fn new(database_url: &str) -> Self
    where
        Self: DatabasePooling,
    {
        let pool = Self::build_pool(database_url, 5).await;
        Self { pool }
    }
}

#[cfg(feature = "diesel")]
#[async_trait::async_trait]
impl DatabasePooling for DieselAsync {
    type ConnectionManager = async_bb8_diesel::ConnectionManager<diesel::PgConnection>;

    type ConnectionPool = bb8::Pool<Self::ConnectionManager>;

    type PooledConnection<'a> = bb8::PooledConnection<'a, Self::ConnectionManager>;

    #[allow(clippy::expect_used)]
    async fn build_pool(database_url: &str, max_size: u32) -> Self::ConnectionPool {
        let manager = Self::ConnectionManager::new(database_url);
        let pool = Self::ConnectionPool::builder().max_size(max_size);
        pool.build(manager)
            .await
            .expect("Failed to create PostgreSQL connection pool")
    }

    #[allow(clippy::expect_used)]
    async fn get_connection(&self) -> Self::PooledConnection<'_> {
        self.pool
            .get()
            .await
            .expect("Couldn't retrieve PostgreSQL connection")
    }
}

#[cfg(feature = "sqlx")]
#[derive(Clone)]
pub struct SqlxAsync {
    pool: sqlx::Pool<sqlx::Postgres>,
}
#[cfg(feature = "sqlx")]
impl SqlxAsync {
    pub async fn new(database_url: &str) -> Self
    where
        Self: DatabasePooling,
    {
        let pool = Self::build_pool(database_url, 5).await;
        Self { pool }
    }
}

#[cfg(feature = "sqlx")]
#[async_trait::async_trait]
impl DatabasePooling for SqlxAsync {
    type ConnectionManager = sqlx::postgres::PgPoolOptions;
    type ConnectionPool = sqlx::Pool<sqlx::Postgres>;
    type PooledConnection<'a> = &'a sqlx::Pool<sqlx::Postgres>;

    #[allow(clippy::expect_used)]
    async fn build_pool(database_url: &str, max_size: u32) -> Self::ConnectionPool {
        Self::ConnectionManager::new()
            .max_connections(max_size)
            .connect(database_url)
            .await
            .expect("Failed to create PostgreSQL connection pool")
    }

    #[allow(clippy::expect_used)]
    async fn get_connection(&self) -> Self::PooledConnection<'_> {
        &self.pool
    }
}

// impl From<sqlx::Pool<sqlx::Postgres>> for SqlxAsync {
//     fn from(value: sqlx::Pool<sqlx::Postgres>) -> Self {
//         Self {
//             pool: value
//         }
//     }
// }
