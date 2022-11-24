use super::*;
use crate::data_layer::ynab_api::response_models as rm;
use crate::*;
use sqlite::{Result, Statement};

pub struct CategoryGroup {
    id: String,
    name: String,
    hidden: bool,
}

impl_insertable!(
    CategoryGroup,
    "../database/queries/category_group/insert.sql"
);
impl_updateable!(
    CategoryGroup,
    "../database/queries/category_group/update.sql"
);

impl From<rm::CategoryGroup> for CategoryGroup {
    fn from(a: rm::CategoryGroup) -> Self {
        Self {
            id: a.id,
            name: a.name,
            hidden: a.hidden,
        }
    }
}

impl BindToStatement for CategoryGroup {
    fn bind(self, s: &mut Statement) -> Result<()> {
        s.bind_iter::<_, (_, Value)>([
            (":id", self.id.into()),
            (":name", self.name.into()),
            (":hidden", self.hidden.into_value()),
        ])
    }
}

impl ReadFromStatement for CategoryGroup {
    fn read(s: &mut Statement) -> Result<Self> {
        Ok(Self {
            id: s.read("id")?,
            name: s.read("name")?,
            hidden: s.read::<i64, _>("budget_id")? != 0,
        })
    }
}
