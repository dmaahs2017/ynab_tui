use crate::data_layer::ynab_api::response_models as rm;
use crate::*;
use super::*;
use sqlite::Statement;

pub struct Category {
    id: String,
    category_group_id: String,
    name: String,
    hidden: bool,
    original_category_group_id: Option<String>,
    note: Option<String>,
    budgeted: i64,
    activity: i64,
    balance: i64,
}

impl_insertable!(Category, "../database/queries/category/insert.sql");
impl_updateable!(Category, "../database/queries/category/update.sql");
impl_id_selectable!(Category, "../database/queries/category/select_by_id.sql");

impl BindToStatement for Category {
    fn bind(self, s: &mut Statement) -> Result<()> {
        s.bind_iter::<_, (_, Value)>([
            (":id", self.id.into()),
            (":category_group_id", self.category_group_id.into()),
            (":name", self.name.into()),
            (":hidden", self.hidden.into_value()),
            (":original_category_group_id", self.original_category_group_id.into_value()),
            (":note", self.note.into_value()),
            (":budgeted", self.budgeted.into()),
            (":activity", self.activity.into()),
            (":balance", self.balance.into()),
        ])
    }
}

impl From<rm::Category> for Category {
    fn from(a: rm::Category) -> Self {
        Self {
            id: a.id,
            category_group_id: a.category_group_id,
            name: a.name,
            hidden: a.hidden,
            original_category_group_id: a.original_category_group_id,
            note: a.note,
            budgeted: a.budgeted,
            activity: a.activity,
            balance: a.balance,
        }
    }
}
