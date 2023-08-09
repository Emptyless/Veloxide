use crate::infrastructure::web_server::graphql::GraphQlConfiguration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfiguration {
    pub graphql: GraphQlConfiguration,
    pub opa_url: String,
}

#[instrument]
pub async fn load_app_configuration() -> crate::prelude::Result<AppConfiguration> {
    let filename = get_env_variable("CONFIGURATION_FILE_PATH").await?;

    let error_string = format!("Could not open file: {filename}");
    let error_string = error_string.as_str();
    let f = std::fs::File::open(&filename).expect(error_string);

    let application_config: AppConfiguration =
        serde_yaml::from_reader(f).expect("Could not read values.");

    Ok(application_config)
}

#[instrument]
pub async fn get_env_variable(variable_name: &str) -> crate::prelude::Result<String> {
    let value = match dotenvy::var(variable_name) {
        Ok(val) => val,
        Err(err) => {
            return Err(crate::error::Error::Generic(format!(
                "Could not read {variable_name}: {err}"
            )));
        }
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn parse_serve_address_parses_correctly() {
        let config = GraphQlConfiguration {
            enabled: true,
            port: 8080,
        };
        let serve_address = config.parse_serve_address();
        assert_eq!(
            serve_address,
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_get_configuration_file_path_variable() {
        env::set_var("CONFIGURATION_FILE_PATH", "test");
        let filename = get_env_variable("CONFIGURATION_FILE_PATH").await.unwrap();
        assert_eq!(filename, "test");
    }
}
