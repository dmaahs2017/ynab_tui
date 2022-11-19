#[derive(Debug)]
pub struct Budget {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
    pub date_format: String,
}

pub struct Transaction {
    pub id: String,
    pub budget_id: String,
    pub date: String, //date string The transaction date in ISO format (e.g. 2016-12-01)
    pub amount: i64,
    pub memo: Option<String>,
    pub account_id: String,                  //Unique
    pub payee_id: Option<String>,            //Unique
    pub category_id: Option<String>,                 //Unique
    pub transfer_account_id: Option<String>, //Unique
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub deleted: bool,
    pub account_name: String,
    pub payee_name: Option<String>,
    pub category_name: String,
}
