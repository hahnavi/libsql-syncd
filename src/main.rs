use libsql::Builder;
use log::{debug, error, info};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let host_id = env::var("HOST_ID").unwrap_or_default();
    let libsql_url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let libsql_auth_token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();
    let libsql_replica_path =
        env::var("LIBSQL_REPLICA_PATH").expect("LIBSQL_REPLICA_PATH must be set");

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_channel = env::var("REDIS_CHANNEL").unwrap_or_else(|_| "libsql-syncd".to_string());

    let db = Builder::new_remote_replica(libsql_replica_path, libsql_url, libsql_auth_token)
        .build()
        .await
        .unwrap();

    db.connect().unwrap();

    db.sync().await.unwrap();

    let redis_client = redis::Client::open(redis_url).unwrap();
    let mut redis_conn = redis_client.get_connection().unwrap();
    let mut pubsub = redis_conn.as_pubsub();

    pubsub.subscribe(redis_channel).unwrap();

    let is_syncing = Arc::new(AtomicBool::new(false));
    let db_arc = Arc::new(db);

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        if payload == host_id {
            debug!(
                "New message matched with HOST_ID: {:?}. Skipping sync.",
                host_id
            );
        } else {
            let is_syncing_clone = is_syncing.clone();
            if !is_syncing_clone.load(Ordering::Relaxed) {
                let db_clone = db_arc.clone();
                tokio::spawn(async move {
                    debug!("Syncing...");
                    is_syncing_clone.store(true, Ordering::Relaxed);
                    match db_clone.sync().await {
                        Ok(_) => {
                            info!("Synced successfully.");
                        }
                        Err(_) => {
                            error!("Failed to sync.");
                        }
                    }
                    sleep(Duration::from_secs(10)).await;
                    is_syncing_clone.store(false, Ordering::Relaxed);
                });
            } else {
                info!("Already syncing...");
            }
        }
    }
}
