pub async fn new_redis_client() -> Result<redis::Client, redis::RedisError> {
    let redis_connection_string = dotenvy::var("REDIS_CONNECTION_STRING")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    redis::Client::open(redis_connection_string)
}
