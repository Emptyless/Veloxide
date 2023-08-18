use crate::interfaces::bank_account::bank_account_views::AccountTransaction as DomainAccountTransaction;
use crate::interfaces::bank_account::bank_account_views::BankAccountView as DomainBankAccountView;
use bank_account_service::bank_account_service_server::BankAccountService;
use bank_account_service::AccountTransaction as GrpcAccountTransaction;
use bank_account_service::BankAccountView as GrpcBankAccountView;
use bank_account_service::{GetBankAccountRequest, GetBankAccountResponse};
use tonic::{Request, Response, Status};

pub mod bank_account_service {
    tonic::include_proto!("bank_account_service");
}
pub use bank_account_service::*;

use crate::application::{BankAccountApplicationService, BankAccountServiceError};

pub struct GRpcBankAccountService {
    app_service: Box<dyn BankAccountApplicationService>,
}

impl GRpcBankAccountService {
    pub fn new(app_service: Box<dyn BankAccountApplicationService>) -> Self {
        GRpcBankAccountService { app_service }
    }
}

#[tonic::async_trait]
impl BankAccountService for GRpcBankAccountService {
    #[tracing::instrument(skip(self), ret, err)]
    async fn get_bank_account(
        &self,
        request: Request<GetBankAccountRequest>,
    ) -> Result<Response<GetBankAccountResponse>, Status> {
        let account_id = request.into_inner().id;

        let account_view = self.app_service.get_bank_account(account_id).await?;
        let reply = bank_account_service::GetBankAccountResponse {
            account_view: Some(account_view.into()),
        };
        Ok(Response::new(reply))
    }
}

const GENERIC_ERROR: &str = "An internal error occurred";
impl From<BankAccountServiceError> for tonic::Status {
    fn from(error: BankAccountServiceError) -> Self {
        match error {
            BankAccountServiceError::Persistence(_) => Status::internal(GENERIC_ERROR),
            BankAccountServiceError::BankAccountNotFound(err) => Status::not_found(err),
        }
    }
}

impl From<DomainBankAccountView> for GrpcBankAccountView {
    fn from(src: DomainBankAccountView) -> Self {
        GrpcBankAccountView {
            account_id: src.account_id.unwrap_or_default(), //TODO: Investigate why this is an option type, ideally removing it
            balance: src.balance,
            written_checks: src.written_checks,
            account_transactions: src
                .account_transactions
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<DomainAccountTransaction> for GrpcAccountTransaction {
    fn from(src: DomainAccountTransaction) -> Self {
        GrpcAccountTransaction {
            amount: src.amount,
            description: src.description,
        }
    }
}
