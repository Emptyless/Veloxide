use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Repository {
    Postgres,
    Memory,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphQlConfiguration {
    pub enabled: bool,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfiguration {
    pub repository: Repository,
    pub graphql: GraphQlConfiguration,
}
