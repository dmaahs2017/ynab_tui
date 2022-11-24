use serde::{Deserialize, Serialize};

use crate::data_layer::models::enums::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct HybridTransaction {
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ClearedStatus,
    pub approved: bool,
    pub flag_color: Option<FlagColor>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: String,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: bool,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub parent_transaction_id: Option<String>,
    pub account_name: Option<String>,
    pub payee_name: Option<String>,
    pub category_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HybridTransactionsResponse {
    pub transactions: Vec<HybridTransaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDetail {
    pub id: String,
    pub date: String, //date string The transaction date in ISO format (e.g. 2016-12-01)
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ClearedStatus,
    pub approved: bool,
    pub flag_color: Option<FlagColor>,
    pub account_id: String,                  //Unique
    pub payee_id: Option<String>,            //Unique
    pub category_id: Option<String>,         //Unique
    pub transfer_account_id: Option<String>, //Unique
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: bool,
    pub account_name: String,
    pub payee_name: Option<String>,
    pub category_name: String,
    pub subtransactions: Vec<SubTransaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTransaction {
    pub id: String,
    pub transaction_id: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub payee_id: Option<String>, //Unique
    pub payee_name: Option<String>,
    pub category_id: Option<String>, //Unique
    pub category_name: String,
    pub transfer_account_id: Option<String>, //Unique
    pub transfer_transaction_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsResponse {
    pub transactions: Vec<TransactionDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub category_group_id: String,
    pub name: String,
    pub hidden: bool,
    pub original_category_group_id: Option<String>,
    pub note: Option<String>,
    pub budgeted: i64,
    pub activity: i64,
    pub balance: i64,
    pub goal_type: Option<GoalType>,
    pub goal_creation_month: Option<String>,
    pub goal_target: Option<i64>,
    pub goal_target_month: Option<String>,
    pub goal_percentage_complete: Option<i32>,
    pub goal_months_to_budget: Option<i32>,
    pub goal_under_funded: Option<i64>,
    pub goal_overall_funded: Option<i64>,
    pub goal_overall_left: Option<i64>,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGroupWithCategories {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub deleted: bool,
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesResponse {
    pub category_groups: Vec<CategoryGroupWithCategories>,
    pub server_knowledge: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateFormat {
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyFormat {
    pub iso_code: String,
    pub example_format: String,
    pub decimal_digits: i32,
    pub decimal_separator: String,
    pub symbol_first: bool,
    pub group_separator: String,
    pub currency_symbol: String,
    pub display_symbol: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: AccountTypeString,
    pub on_budget: bool,
    pub closed: bool,
    pub note: String,
    pub balance: i64,
    pub cleared_balance: i64,
    pub uncleared_balance: i64,
    pub transfer_payee_id: Option<String>,
    pub direct_import_linked: bool,
    pub direct_import_in_error: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetSummary {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
    pub date_format: DateFormat,
    pub currency_format: CurrencyFormat,
    pub accounts: Option<Vec<Account>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetSummaryResponse {
    pub budgets: Vec<BudgetSummary>,
    pub default_budget: Option<BudgetSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payee {
    pub id: String,
    pub name: String,
    pub transfer_account_id: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayeeLocation {
    pub id: String,
    pub payee_id: String,
    pub latitude: String,
    pub longitude: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGroup {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthDetail {
    pub month: String,
    pub note: String,
    pub income: i64,
    pub budgeted: i64,
    pub activity: i64,
    pub to_be_budgeted: i64,
    pub age_of_money: i32,
    pub deleted: bool,
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: ClearedStatus,
    pub approved: bool,
    pub flag_color: FlagColor,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledTransactionSummary {
    pub id: String,
    pub date_first: String,
    pub date_next: String,
    pub frequency: Frequency,
    pub amount: i64,
    pub memo: String,
    pub flag_color: FlagColor,
    pub account_id: String,
    pub payee_id: String,
    pub category_id: String,
    pub transfer_account_id: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledSubTransaction {
    pub id: String,
    pub scheduled_transaction_id: String,
    pub amount: i64,
    pub memo: String,
    pub payee_id: String,
    pub category_id: String,
    pub transfer_account_id: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetDetail {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
    pub date_format: DateFormat,
    pub currency_format: CurrencyFormat,
    pub accounts: Vec<Account>,
    pub payees: Vec<Payee>,
    pub payee_locations: Vec<PayeeLocation>,
    pub category_groups: Vec<CategoryGroup>,
    pub categories: Vec<Category>,
    pub months: Vec<MonthDetail>,
    pub transactions: Vec<TransactionSummary>,
    pub sub_transactions: Vec<SubTransaction>,
    pub scheduled_transactions: Vec<ScheduledTransactionSummary>,
    pub scheduled_subtransactions: Vec<ScheduledSubTransaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetDetailResponse {
    pub budget: BudgetDetail,
    pub server_knowledge: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data<T> {
    pub data: T,
}
