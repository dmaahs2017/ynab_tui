/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScheduledTransactionDetail {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The first date for which the Scheduled Transaction was scheduled.
    #[serde(rename = "date_first")]
    pub date_first: String,
    /// The next date for which the Scheduled Transaction is scheduled.
    #[serde(rename = "date_next")]
    pub date_next: String,
    #[serde(rename = "frequency")]
    pub frequency: Frequency,
    /// The scheduled transaction amount in milliunits format
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "memo", skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// The scheduled transaction flag
    #[serde(rename = "flag_color", skip_serializing_if = "Option::is_none")]
    pub flag_color: Option<FlagColor>,
    #[serde(rename = "account_id")]
    pub account_id: uuid::Uuid,
    #[serde(rename = "payee_id", skip_serializing_if = "Option::is_none")]
    pub payee_id: Option<uuid::Uuid>,
    #[serde(rename = "category_id", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<uuid::Uuid>,
    /// If a transfer, the account_id which the scheduled transaction transfers to
    #[serde(rename = "transfer_account_id", skip_serializing_if = "Option::is_none")]
    pub transfer_account_id: Option<uuid::Uuid>,
    /// Whether or not the scheduled transaction has been deleted.  Deleted scheduled transactions will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "account_name")]
    pub account_name: String,
    #[serde(rename = "payee_name", skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,
    /// The name of the category.  If a split scheduled transaction, this will be 'Split'.
    #[serde(rename = "category_name", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    /// If a split scheduled transaction, the subtransactions.
    #[serde(rename = "subtransactions")]
    pub subtransactions: Vec<crate::models::ScheduledSubTransaction>,
}

impl ScheduledTransactionDetail {
    pub fn new(id: uuid::Uuid, date_first: String, date_next: String, frequency: Frequency, amount: i64, account_id: uuid::Uuid, deleted: bool, account_name: String, subtransactions: Vec<crate::models::ScheduledSubTransaction>) -> ScheduledTransactionDetail {
        ScheduledTransactionDetail {
            id,
            date_first,
            date_next,
            frequency,
            amount,
            memo: None,
            flag_color: None,
            account_id,
            payee_id: None,
            category_id: None,
            transfer_account_id: None,
            deleted,
            account_name,
            payee_name: None,
            category_name: None,
            subtransactions,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Frequency {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "everyOtherWeek")]
    EveryOtherWeek,
    #[serde(rename = "twiceAMonth")]
    TwiceAMonth,
    #[serde(rename = "every4Weeks")]
    Every4Weeks,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "everyOtherMonth")]
    EveryOtherMonth,
    #[serde(rename = "every3Months")]
    Every3Months,
    #[serde(rename = "every4Months")]
    Every4Months,
    #[serde(rename = "twiceAYear")]
    TwiceAYear,
    #[serde(rename = "yearly")]
    Yearly,
    #[serde(rename = "everyOtherYear")]
    EveryOtherYear,
}

impl Default for Frequency {
    fn default() -> Frequency {
        Self::Never
    }
}
/// The scheduled transaction flag
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlagColor {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "purple")]
    Purple,
}

impl Default for FlagColor {
    fn default() -> FlagColor {
        Self::Red
    }
}

