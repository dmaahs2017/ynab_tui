/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountType : The type of account

/// The type of account
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "creditCard")]
    CreditCard,
    #[serde(rename = "lineOfCredit")]
    LineOfCredit,
    #[serde(rename = "otherAsset")]
    OtherAsset,
    #[serde(rename = "otherLiability")]
    OtherLiability,
    #[serde(rename = "mortgage")]
    Mortgage,
    #[serde(rename = "autoLoan")]
    AutoLoan,
    #[serde(rename = "studentLoan")]
    StudentLoan,
    #[serde(rename = "personalLoan")]
    PersonalLoan,
    #[serde(rename = "medicalDebt")]
    MedicalDebt,
    #[serde(rename = "otherDebt")]
    OtherDebt,

}

impl ToString for AccountType {
    fn to_string(&self) -> String {
        match self {
            Self::Checking => String::from("checking"),
            Self::Savings => String::from("savings"),
            Self::Cash => String::from("cash"),
            Self::CreditCard => String::from("creditCard"),
            Self::LineOfCredit => String::from("lineOfCredit"),
            Self::OtherAsset => String::from("otherAsset"),
            Self::OtherLiability => String::from("otherLiability"),
            Self::Mortgage => String::from("mortgage"),
            Self::AutoLoan => String::from("autoLoan"),
            Self::StudentLoan => String::from("studentLoan"),
            Self::PersonalLoan => String::from("personalLoan"),
            Self::MedicalDebt => String::from("medicalDebt"),
            Self::OtherDebt => String::from("otherDebt"),
        }
    }
}

impl Default for AccountType {
    fn default() -> AccountType {
        Self::Checking
    }
}




