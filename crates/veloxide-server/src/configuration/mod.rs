pub mod config;
pub mod db_connection;
pub mod observability;
pub mod redis_connection;

// Re-exports
pub use config::*;
pub use db_connection::*;
pub use redis_connection::*;
pub use tracing::*;

use tracing::{instrument, Level};

#[instrument]
pub async fn load_app_configuration() -> crate::prelude::Result<AppConfiguration> {
    let filename = get_configuration_file_path_variable("CONFIGURATION_FILE_PATH").await?;

    let error_string = format!("Could not open file: {filename}");
    let error_string = error_string.as_str();
    let f = std::fs::File::open(&filename).expect(error_string);

    let application_config: AppConfiguration =
        serde_yaml::from_reader(f).expect("Could not read values.");

    Ok(application_config)
}

#[instrument]
pub async fn get_configuration_file_path_variable(
    variable_name: &str,
) -> crate::prelude::Result<String> {
    let filename = match dotenvy::var(variable_name) {
        Ok(val) => val,
        Err(err) => {
            return Err(crate::error::Error::Generic(format!(
                "Could not read {variable_name}: {err}"
            )));
        }
    };
    Ok(filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_get_configuration_file_path_variable() {
        env::set_var("CONFIGURATION_FILE_PATH", "test");
        let filename = get_configuration_file_path_variable("CONFIGURATION_FILE_PATH")
            .await
            .unwrap();
        assert_eq!(filename, "test");
    }
}
