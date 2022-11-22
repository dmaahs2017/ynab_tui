pub mod response_models;
use response_models::*;

use chrono::{DateTime, Duration, Local};
use reqwest::blocking::Client;
use reqwest::{header, header::HeaderMap};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io};

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

    fn get(&mut self, endpoint: &str) -> ApiResult<String> {
        if let Some(cache_entry) = self.cache.get(endpoint) {
            if Local::now() - cache_entry.datetime < self.refresh_duration && !self.force_refresh {
                self.cache_hit += 1;
                return Ok(cache_entry.response_json.clone());
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
        Ok(resp)
    }

    /// TODO: Implement include accounts feature
    pub fn list_budgets(
        &mut self,
        _include_accounts: bool,
    ) -> ApiResult<Data<BudgetSummaryResponse>> {
        let endp = "/budgets";
        Ok(serde_json::from_str(&self.get(endp)?)?)
    }

    // TODO: Implement last knowledge of server
    pub fn budget_export(
        &mut self,
        budget_id: &str,
        _last_knowledge_of_server: Option<i32>,
    ) -> ApiResult<Data<BudgetDetailResponse>> {
        let endp = &format!("/budgets/{budget_id}");
        Ok(serde_json::from_str(&self.get(endp)?)?)
    }

    /// TODO
    #[allow(dead_code)]
    pub fn budget_settings(&mut self, _budget_id: &str) -> ApiResult<Data<()>> {
        todo!("GET /budgets/{_budget_id}/settings")
    }

    /// TODO
    #[allow(dead_code)]
    pub fn list_accounts(
        &mut self,
        _budget_id: &str,
        _last_knowledge_of_server: Option<i32>,
    ) -> ApiResult<Data<()>> {
        todo!("GET /budgets/{_budget_id}/accounts")
    }

    /// TODO
    #[allow(dead_code)]
    pub fn create_account(
        &mut self,
        _budget_id: &str,
        _name: &str,
        _type: &str,
        _balance: i64,
    ) -> ApiResult<Data<()>> {
        todo!("POST /budgets/{_budget_id}/accounts")
    }

    /// TODO
    #[allow(dead_code)]
    pub fn get_account(&mut self, _budget_id: &str, _account_id: &str) -> ApiResult<Data<()>> {
        todo!("GET /budgets/{_budget_id}/accounts/{_account_id}")
    }

    #[allow(dead_code)]
    pub fn list_categories(&mut self, budget_id: &str) -> ApiResult<Data<CategoriesResponse>> {
        let endp = &format!("/budgets/{budget_id}/categories");
        Ok(serde_json::from_str(&self.get(endp)?)?)
    }

    #[allow(dead_code)]
    pub fn get_category_transactions(
        &mut self,
        budget_id: &str,
        category_id: &str,
    ) -> ApiResult<Data<HybridTransactionsResponse>> {
        let endp = &format!("/budgets/{budget_id}/categories/{category_id}/transactions");
        Ok(serde_json::from_str(&self.get(endp)?)?)
    }

    //TODO: Implement since_date, and trans_type
    pub fn get_budget_transactions(
        &mut self,
        budget_id: &str,
        _since_date: Option<DateTime<Local>>,
        _trans_type: Option<String>,
        last_knowledge: Option<usize>,
    ) -> ApiResult<Data<TransactionsResponse>> {
        let endp = &if let Some(lk) = last_knowledge {
            format!("/budgets/{budget_id}/transactions?last_knowledge_of_server={lk}")
        } else {
            format!("/budgets/{budget_id}/transactions")
        };

        Ok(serde_json::from_str(&self.get(endp)?)?)
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
