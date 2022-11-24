use super::*;
use crate::data_layer::ynab_api::response_models as rs;
use crate::*;

use sqlite::Statement;

pub struct Account {
    id: String,
    name: String,
    account_type: AccountTypeString,
    on_budget: bool,
    closed: bool,
    note: String,
    balance: i64,
    cleared_balance: i64,
    uncleared_balance: i64,
    transfer_payee_id: Option<String>,
    direct_import_linked: bool,
    direct_import_in_error: bool,
}

impl_insertable!(Account, "../database/queries/account/insert.sql");
impl_updateable!(Account, "../database/queries/account/update.sql");

impl BindToStatement for Account {
    fn bind(self, s: &mut Statement) -> Result<()> {
        s.bind_iter::<_, (_, Value)>([
            (":id", self.id.into()),
            (":name", self.name.into()),
            (":account_type", self.account_type.into_value()),
            (":on_budget", self.on_budget.into_value()),
            (":balance", self.balance.into()),
            (":cleared_balance", self.cleared_balance.into()),
            (":uncleared_balance", self.uncleared_balance.into()),
            (":transfer_payee_id", self.transfer_payee_id.into_value()),
            (
                ":direct_import_linked",
                self.direct_import_linked.into_value(),
            ),
            (
                ":direct_import_in_error",
                self.direct_import_in_error.into_value(),
            ),
        ])
    }
}

impl From<rs::Account> for Account {
    fn from(a: rs::Account) -> Self {
        Self {
            id: a.id,
            name: a.name,
            account_type: a.account_type,
            on_budget: a.on_budget,
            cleared_balance: a.cleared_balance,
            uncleared_balance: a.uncleared_balance,
            closed: a.closed,
            note: a.note,
            balance: a.balance,
            transfer_payee_id: a.transfer_payee_id,
            direct_import_linked: a.direct_import_linked,
            direct_import_in_error: a.direct_import_linked,
        }
    }
}
