use crate::data_layer::ynab_api::response_models as rm;
use crate::*;

use super::*;
use sqlite::{Result, Statement};

#[derive(Debug, Clone)]
pub struct Budget {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
    pub date_format: String,
}

impl_insertable!(Budget, "../database/queries/budget/insert.sql");
impl_updateable!(Budget, "../database/queries/budget/update.sql");
impl_id_selectable!(Budget, "../database/queries/budget/select_by_id.sql");
impl_all_selectable!(Budget, "../database/queries/budget/select_all.sql");

impl From<rm::BudgetSummary> for Budget {
    fn from(a: rm::BudgetSummary) -> Self {
        Self {
            id: a.id,
            name: a.name,
            last_modified_on: a.last_modified_on,
            first_month: a.first_month,
            last_month: a.last_month,
            date_format: a.date_format.format,
        }
    }
}

impl ReadFromStatement for Budget {
    fn read(s: &mut Statement) -> Result<Self> {
        Ok(Budget {
            id: s.read("id")?,
            name: s.read("name")?,
            last_modified_on: s.read("last_modified_on")?,
            first_month: s.read("first_month")?,
            last_month: s.read("last_month")?,
            date_format: s.read("date_format")?,
        })
    }
}

impl BindToStatement for Budget {
    fn bind(self, s: &mut Statement) -> Result<()> {
        s.bind_iter::<_, (_, Value)>([
            (":id", self.id.into()),
            (":name", self.name.into()),
            (":last_modified_on", self.last_modified_on.into()),
            (":first_month", self.first_month.into()),
            (":last_month", self.last_month.into()),
            (":date_format", self.date_format.into()),
        ])
    }
}
