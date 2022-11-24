use super::*;
use crate::data_layer::ynab_api::response_models as rm;
use crate::*;
use sqlite::Result;
use sqlite::Statement;

#[derive(PartialEq, Eq, Clone)]
pub struct Transaction {
    pub id: String,
    pub budget_id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub account_id: String,                  //Unique
    pub payee_id: Option<String>,            //Unique
    pub category_id: Option<String>,         //Unique
    pub transfer_account_id: Option<String>, //Unique
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
}

impl_insertable!(Transaction, "../database/queries/transaction/insert.sql");
impl_updateable!(Transaction, "../database/queries/transaction/update.sql");
impl_id_selectable!(
    Transaction,
    "../database/queries/transaction/select_by_id.sql"
);

impl BindToStatement for Transaction {
    fn bind(self, s: &mut Statement) -> Result<()> {
        s.bind_iter::<_, (_, Value)>([
            (":id", self.id.into()),
            (":budget_id", self.budget_id.into()),
            (":date", self.date.into()),
            (":amount", self.amount.into()),
            (":memo", self.memo.into_value()),
            (":account_id", self.account_id.into()),
            (":payee_id", self.payee_id.into_value()),
            (":category_id", self.category_id.into_value()),
            (
                ":transfer_account_id",
                self.transfer_account_id.into_value(),
            ),
            (
                ":transfer_transaction_id",
                self.transfer_transaction_id.into_value(),
            ),
            (
                ":matched_transaction_id",
                self.matched_transaction_id.into_value(),
            ),
        ])
    }
}

impl ReadFromStatement for Transaction {
    fn read(s: &mut Statement) -> Result<Self> {
        Ok(Self {
            id: s.read("id")?,
            budget_id: s.read("budget_id")?,
            date: s.read("date")?,
            amount: s.read("amount")?,
            memo: s.read("memo")?,
            account_id: s.read("account_id")?,
            payee_id: s.read("payee_id")?,
            category_id: s.read("category_id")?,
            transfer_account_id: s.read("transfer_account_id")?,
            transfer_transaction_id: s.read("transfer_transaction_id")?,
            matched_transaction_id: s.read("matched_transaction_id")?,
        })
    }
}

impl Transaction {
    pub fn from_detail(t: rm::TransactionDetail, b_id: &str) -> Self {
        Self {
            id: t.id,
            budget_id: b_id.to_string(),
            date: t.date,
            amount: t.amount,
            memo: t.memo,
            account_id: t.account_id,
            payee_id: t.payee_id,
            category_id: t.category_id,
            transfer_account_id: t.transfer_account_id,
            transfer_transaction_id: t.transfer_transaction_id,
            matched_transaction_id: t.matched_transaction_id,
        }
    }

    pub fn from_summary(t: rm::TransactionSummary, b_id: &str) -> Self {
        Self {
            id: t.id,
            budget_id: b_id.to_string(),
            date: t.date,
            amount: t.amount,
            memo: t.memo,
            account_id: t.account_id,
            payee_id: t.payee_id,
            category_id: t.category_id,
            transfer_account_id: t.transfer_account_id,
            transfer_transaction_id: t.transfer_transaction_id,
            matched_transaction_id: t.matched_transaction_id,
        }
    }
}
