use chrono::{DateTime, Duration, Local};
use reqwest::blocking::Client;
use reqwest::{header, header::HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};
use std::{collections::HashMap, fs, io};

use super::models::{Account, Budget, Transaction};

type ApiResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct YnabApi {
    base_url: String,
    cache: HashMap<String, CacheEntry>,
    cache_hit: u32,
    cache_miss: u32,
    client: Client,
    token: String,
    cache_file: String,
    refresh_duration: Duration,
    force_refresh: bool,
}

impl YnabApi {
    pub fn new(token: &str, cache_file: &str, refresh_duration: Duration) -> Self {
        let mut cache = fs::File::open(cache_file)
            .map(|f| serde_json::from_reader(io::BufReader::new(f)).unwrap_or_default())
            .unwrap_or({
                let mut map = HashMap::new();
                map.insert(
                    "created".to_string(),
                    CacheEntry {
                        datetime: Local::now(),
                        response_json: String::new(),
                    },
                );
                map
            });

        if let Some(entry) = cache.get("created") {
            if Local::now() - entry.datetime >= Duration::days(1) {
                cache.clear();
                cache.insert(
                    "created".to_string(),
                    CacheEntry {
                        datetime: Local::now(),
                        response_json: String::new(),
                    },
                );
            }
        } else {
            cache.clear();
            cache.insert(
                "created".to_string(),
                CacheEntry {
                    datetime: Local::now(),
                    response_json: String::new(),
                },
            );
        }

        Self {
            base_url: String::from("https://api.youneedabudget.com/v1"),
            cache,
            cache_hit: 0,
            cache_miss: 0,
            client: Client::new(),
            token: token.to_string(),
            cache_file: cache_file.to_string(),
            refresh_duration,
            force_refresh: false,
        }
    }

    fn get(&mut self, endpoint: &str) -> ApiResult<JsonValue> {
        if let Some(cache_entry) = self.cache.get(endpoint) {
            if Local::now() - cache_entry.datetime < self.refresh_duration && !self.force_refresh {
                self.cache_hit += 1;
                let serde_json = serde_json::from_str(&cache_entry.response_json)?;
                return Ok(serde_json);
            }
        }
        self.cache_miss += 1;
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.token)
                .parse()
                .expect("Created auth header"),
        );
        let resp = self
            .client
            .get(format!("{}{}", self.base_url, endpoint))
            .headers(headers)
            .send()?
            .text()?;

        self.cache.insert(
            endpoint.to_string(),
            CacheEntry {
                datetime: Local::now(),
                response_json: resp.clone(),
            },
        );

        Ok(serde_json::from_str(&resp)?)
    }

    /// TODO: Implement include accounts feature
    pub fn list_budgets(&mut self) -> ApiResult<Vec<Budget>> {
        let endp = "/budgets";
        let response = self.get(endp)?;

        Ok(response["data"]["budgets"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|bj| Budget {
                id: bj["id"].as_str().unwrap().to_string(),
                name: bj["name"].as_str().unwrap().to_string(),
                last_modified_on: bj["last_modified_on"].as_str().unwrap().to_string(),
                first_month: bj["first_month"].as_str().unwrap().to_string(),
                last_month: bj["last_month"].as_str().unwrap().to_string(),
                date_format: bj["date_format"]["format"].as_str().unwrap().to_string(),
            })
            .collect())
    }

    pub fn list_accounts(&mut self, budget_id: &str) -> ApiResult<Vec<Account>> {
        let endp = &format!("/budgets/{budget_id}/accounts");
        let response = self.get(endp)?;

        Ok(response["data"]["accounts"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|bj| Account {
                id: bj["id"].as_str().unwrap().to_string(),
                name: bj["name"].as_str().unwrap().to_string(),
            })
            .collect())
    }

    pub fn list_account_transactions(
        &mut self,
        budget_id: &str,
        account_id: &str,
    ) -> ApiResult<Vec<Transaction>> {
        let endp = &format!("/budgets/{budget_id}/accounts/{account_id}/transactions");
        let response = self.get(endp)?;

        let mut ts = response["data"]["transactions"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|json| transaction_from_json(json))
            .collect::<Vec<_>>();

        ts.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(ts)
    }

    pub fn list_transactions(
        &mut self,
        budget_id: &str,
        last_knowledge: Option<usize>,
    ) -> ApiResult<Vec<Transaction>> {
        let endp = &if let Some(lk) = last_knowledge {
            format!("/budgets/{budget_id}/transactions?last_knowledge_of_server={lk}")
        } else {
            format!("/budgets/{budget_id}/transactions")
        };

        let response = self.get(endp)?;
        let mut ts = response["data"]["transactions"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|json| transaction_from_json(json))
            .collect::<Vec<_>>();
        ts.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(ts)
    }
}

fn transaction_from_json(json: &JsonValue) -> Transaction {
    Transaction {
        id: json["id"].as_str().unwrap().to_string(),
        date: json["date"].as_str().unwrap().to_string(),
        amount: json["amount"].as_i64().unwrap(),
        memo: json["memo"].as_str().map(str::to_string),
        account_id: json["account_id"].as_str().unwrap().to_string(),
        account_name: json["account_name"].as_str().unwrap().to_string(),
        payee_id: json["payee_id"].as_str().map(str::to_string),
        category_id: json["category_id"].as_str().map(str::to_string),
        payee_name: json["payee_name"].as_str().map(str::to_string),
        category_name: json["category_name"].as_str().unwrap().to_string(),
    }
}

impl Drop for YnabApi {
    fn drop(&mut self) {
        let f = std::fs::File::create(&self.cache_file).expect("Failed to create cache file");
        let w = std::io::BufWriter::new(f);
        serde_json::to_writer(w, &self.cache).expect("Writing cache to disk failed");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CacheEntry {
    datetime: DateTime<Local>,
    response_json: String,
}
