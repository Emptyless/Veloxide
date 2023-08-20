pub use tracing::*;

#[instrument]
fn get_database_environment_variable() -> String {
    tracing::event!(Level::INFO, "getting database environment variable");
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::event!(Level::INFO, "database environment variable set.");
    db_url
}

cfg_if::cfg_if! {
    if #[cfg(feature = "postgres")] {
        use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
        #[instrument]
        pub async fn get_db_connection_postgres_sqlx() -> crate::prelude::Result<Pool<Postgres>> {
            let db_connection_url = get_database_environment_variable();
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(db_connection_url.as_str())
                .await?;
            sqlx::migrate!()
                .run(&pool)
                .await.expect("failed to run migrations");

            Ok(pool)
        }
        pub async fn get_db_connection() -> crate::prelude::Result<Pool<Postgres>> {
            get_db_connection_postgres_sqlx().await
        }
    } else if #[cfg(feature = "mysql")] {
        use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

        #[instrument]
        pub async fn get_db_connection_mysql_sqlx() -> crate::prelude::Result<Pool<MySql>> {
            let db_connection_url = get_database_environment_variable();
            let pool = MySqlPoolOptions::new()
                .max_connections(5)
                .connect(db_connection_url.as_str())
                .await?;
            sqlx::migrate!()
                .run(&pool)
                .await.expect("failed to run migrations");
            Ok(pool)
        }
        pub async fn get_db_connection() -> crate::prelude::Result<Pool<MySql>> {
            get_db_connection_mysql_sqlx().await
        }
    } else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENV_EXAMPLE_FILEPATH: &str = ".env.example";

    #[tokio::test]
    async fn test_database_url_is_set_in_env_example_correctly() {
        let load_result = dotenvy::from_filename_override(ENV_EXAMPLE_FILEPATH);
        assert!(load_result.is_ok());

        let db_connection_url = get_database_environment_variable().await;
        assert_eq!(
            db_connection_url,
            "postgresql://postgres:thisisnotsecure@localhost:5432/veloxidedb"
        );
    }
}
