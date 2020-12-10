use log::info;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::ConnectOptions;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader};

pub type DBPool = MySqlPool;

// 初始化db连接池
pub async fn init_db() -> DBPool {
    let config = load_yml().unwrap();
    println!("{:?}", config);
    let conf = &config[0];

    let mut opt: sqlx::mysql::MySqlConnectOptions = conf["DATABASE_URL"]
        .as_str()
        .unwrap()
        .parse()
        .expect("could not parse db connection string");

    opt.log_statements(log::LevelFilter::Off);
    opt.log_slow_statements(
        log::LevelFilter::Warn,
        Duration::from_secs(conf["SQL_SLOW_TIME"].as_i64().unwrap() as u64),
    );

    MySqlPoolOptions::new()
        .max_connections(conf["POOL_MAX_CONN"].as_i64().unwrap() as u32)
        .min_connections(conf["POOL_MIN_CONN"].as_i64().unwrap() as u32)
        .connect_timeout(Duration::from_secs(
            conf["POOL_CONN_TIMEOUT"].as_i64().unwrap() as u64,
        ))
        .idle_timeout(Duration::from_secs(
            conf["POOL_IDLE_TIMEOUT"].as_i64().unwrap() as u64,
        ))
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

fn load_yml() -> std::io::Result<Vec<Yaml>> {
    let mut file = File::open("conf.yml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let conf = YamlLoader::load_from_str(&contents).unwrap();
    Ok(conf)
}
