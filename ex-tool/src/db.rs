use log::info;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::ConnectOptions;
use std::time::Duration;
pub type DBPool = MySqlPool;

#[derive(Debug)]
pub struct Router {
    pub server_id: i64,
    pub level: i32,
    pub group: i32,
}
// 初始化db连接池
pub async fn init_db() -> DBPool {
    let max_conn: u32 = env2val("POOL_MAX_CONN", 10);
    let min_conn: u32 = env2val("POOL_MIN_CONN", 1);
    let conn_timeout: u64 = env2val("POOL_CONN_TIMEOUT", 10);
    let idle_timeout: u64 = env2val("POOL_IDLE_TIMEOUT", 60);
    let slow_time: u64 = env2val("SQL_SLOW_TIME", 10);

    let mut opt: sqlx::mysql::MySqlConnectOptions = std::env::var("DATABASE_URL")
        .unwrap()
        .parse()
        .expect("could not parse db connection string");
    opt.log_statements(log::LevelFilter::Off);
    opt.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(slow_time));

    MySqlPoolOptions::new()
        .max_connections(max_conn)
        .min_connections(min_conn)
        .connect_timeout(Duration::from_secs(conn_timeout))
        .idle_timeout(Duration::from_secs(idle_timeout))
        .test_before_acquire(true)
        .after_connect(move |_conn| {
            Box::pin(async move {
                info!("db_pool: established a new connection.");
                Ok(())
            })
        })
        .connect_with(opt)
        .await
        .expect("could not create db_pool due to")
}

fn env2val<T>(name: &str, val: T) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    if let Ok(v) = std::env::var(name) {
        v.parse::<T>()
            .expect(&format!("failed to parse env: {:?}", name))
    } else {
        val
    }
}
