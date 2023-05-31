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
pub struct PayeeResponse {
    #[serde(rename = "data")]
    pub data: Box<crate::models::PayeeResponseData>,
}

impl PayeeResponse {
    pub fn new(data: crate::models::PayeeResponseData) -> PayeeResponse {
        PayeeResponse {
            data: Box::new(data),
        }
    }
}


