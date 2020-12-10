mod db;
mod handler;
mod utils;

#[tokio::main]
async fn main() {
    init_env();

    let dbpool = db::init_db().await;
    handler::sync(dbpool).await;
}

fn init_env() {
    dotenv::dotenv().ok();
    utils::init_log();
}
