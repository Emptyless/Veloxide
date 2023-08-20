use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub const FRONTEND_CLIENT_ORIGIN_ENV_VAR: &str = "FRONTEND_CLIENT_ORIGIN";
pub const HTTP_PORT_ENV_VAR: &str = "HTTP_PORT";
pub const DEFAULT_REDIRECT_PATH: &str = "/swagger-ui";
pub const HTTP_PORT_DEFAULT: &str = "8080";

#[derive(Serialize, Deserialize, Clone)]
pub struct WebServerConfiguration {
    pub port: u16,
    pub host: String,
}

impl WebServerConfiguration {
    pub fn from_env() -> WebServerConfiguration {
        let port: u16 = dotenvy::var(HTTP_PORT_ENV_VAR)
            .unwrap_or_else(|_| HTTP_PORT_DEFAULT.to_string())
            .parse()
            .expect("PORT must be a valid integer");
        WebServerConfiguration {
            port,
            host: "[::]".to_string(),
        }
    }

    pub fn get_address(&self) -> SocketAddr {
        format!("{}:{}", &self.host, &self.port).parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const ENV_EXAMPLE_FILEPATH: &str = ".env.example";

    #[tokio::test]
    async fn test_http_port_default_in_env_example_is_set() {
        let load_result = dotenvy::from_filename_override(ENV_EXAMPLE_FILEPATH);
        assert_eq!(load_result.is_ok(), true);

        let http_port = dotenvy::var(HTTP_PORT_ENV_VAR);
        assert_eq!(http_port.unwrap(), HTTP_PORT_DEFAULT.to_string());
    }
}
