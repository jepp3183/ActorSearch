use redis::aio::ConnectionManager;
use redis::AsyncCommands;

pub struct PosterCache {
    connection: ConnectionManager,
}

impl PosterCache {
    pub async fn new(url: &str) -> Self {
        let Ok(client) = redis::Client::open(url) else {
            panic!("Could not connect to redis url");
        };

        let Ok(connection) = client.get_tokio_connection_manager().await else {
            panic!("Could not get connection manager");
        };

        Self { connection }
    }

    pub async fn get(&self, key: &String) -> Option<String> {
        let res = self.connection.clone().get(key).await;
        match res {
            Ok(val) => val,
            Err(_) => None,
        }
    }

    pub async fn set(&self, key: &String, value: &String) {
        let Ok(_) = self.connection
            .clone()
            .set_ex::<&String, &String, String>(key, value, 86400)
            .await else {
                panic!("Could not set key {} to value {}", key, value);
            };
    }
}
