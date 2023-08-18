use cqrs_es::persist::ViewRepository;
use std::sync::Arc;

use async_trait::async_trait;
use postgres_es::PostgresViewRepository;

use crate::domain::BankAccount;
use crate::interfaces::bank_account::bank_account_views::BankAccountView;

#[derive(thiserror::Error, Debug)]
pub enum BankAccountServiceError {
    #[error("bank account not found: {0}")]
    BankAccountNotFound(String),

    #[error(transparent)]
    Persistence(#[from] cqrs_es::persist::PersistenceError),
}

#[async_trait]
pub trait BankAccountApplicationService: Send + Sync {
    async fn get_bank_account(
        &self,
        account_id: String,
    ) -> Result<BankAccountView, BankAccountServiceError>;
}

pub struct BankAccountServiceImpl {
    view_repository: Arc<PostgresViewRepository<BankAccountView, BankAccount>>,
}

impl BankAccountServiceImpl {
    pub fn new(view_repository: Arc<PostgresViewRepository<BankAccountView, BankAccount>>) -> Self {
        Self { view_repository }
    }
}

#[async_trait]
impl BankAccountApplicationService for BankAccountServiceImpl {
    async fn get_bank_account(
        &self,
        account_id: String,
    ) -> Result<BankAccountView, BankAccountServiceError> {
        let view_option = self.view_repository.load(&account_id).await?;
        match view_option {
            Some(view) => Ok(view),
            None => Err(BankAccountServiceError::BankAccountNotFound(account_id)),
        }
    }
}
