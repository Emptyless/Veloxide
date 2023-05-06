const REDIS_CONNECTION_STRING_ENV_VAR: &str = "REDIS_CONNECTION_STRING";
const REDIS_CONNECTION_STRING_DEFAULT: &str = "redis://localhost:6379";

pub async fn new_redis_client() -> Result<redis::Client, redis::RedisError> {
    let redis_connection_string = dotenvy::var(REDIS_CONNECTION_STRING_ENV_VAR)
        .unwrap_or_else(|_| REDIS_CONNECTION_STRING_DEFAULT.to_string());

    redis::Client::open(redis_connection_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const ENV_EXAMPLE_FILEPATH: &str = ".env.example";

    #[tokio::test]
    async fn loading_redis_connection_from_env_example_works() {
        let load_result = dotenvy::from_filename_override(ENV_EXAMPLE_FILEPATH);
        assert_eq!(load_result.is_ok(), true);
        let redis_connection_string = dotenvy::var(REDIS_CONNECTION_STRING_ENV_VAR);
        assert_eq!(
            redis_connection_string.unwrap(),
            REDIS_CONNECTION_STRING_DEFAULT.to_string()
        );
    }
}
