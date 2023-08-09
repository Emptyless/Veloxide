use utoipa::OpenApi;

//TODO: Remove reaching into domain from here
use crate::domain::bank_account::*;
use crate::infrastructure::web_server::bank_account_handlers;
use crate::infrastructure::web_server::oauth::*;
use crate::interfaces::*;

#[derive(OpenApi)]
#[openapi(
      paths(
          bank_account_handlers::query_handler,
          bank_account_handlers::command_handler,
          login,
          logout,
          protected,
      ),
      components(
          schemas(
            BankAccountView,
            BankAccountCommand,
            BankAccountOpenAccountCommandData,
            BankAccountDepositMoneyCommandData,
            BankAccountWithdrawMoneyCommandData,
            BankAccountWriteCheckCommandData,
            AccountTransaction),
    ),
      tags(
          (name = "Bank Accounts", description = "Bank Account Management API")
      ),
        info(
            title = "Bank Account API: built with Veloxide",
            version = "0.1.0",
            description = "An event-sourced bank account API built with Veloxide",
            contact(name = "Liam Woodleigh", url="https://github.com/liamwh/"),
        ),
  )]
pub struct ApiDoc;
