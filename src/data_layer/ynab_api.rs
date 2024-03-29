use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs, io,
};
use ynab_openapi::{
    apis::{
        accounts_api, budgets_api,
        configuration::{ApiKey, Configuration},
        transactions_api,
    },
    models::{Account, BudgetSummary, TransactionDetail},
};

type ApiResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct YnabApi {
    cache: HashMap<String, CacheEntry>,
    config: Configuration,
    cache_hit: u32,
    cache_file: String,
    refresh_duration: Duration,
    force_refresh: bool,
}

impl YnabApi {
    pub fn new(token: &str, cache_file: &str, refresh_duration: Duration) -> Self {
        let cache = fs::File::open(cache_file)
            .map(|f| serde_json::from_reader(io::BufReader::new(f)).unwrap_or_default())
            .unwrap_or_default();

        let mut config = Configuration::new();
        config.bearer_access_token = Some(token.to_string());
        config.api_key = Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: token.to_string(),
        });

        Self {
            cache,
            cache_hit: 0,
            config,
            cache_file: cache_file.to_string(),
            refresh_duration,
            force_refresh: false,
        }
    }

    fn get<'a, 's: 'a, T, F>(&'s mut self, endpoint: String, api_call: F) -> ApiResult<T>
    where
        F: Fn(&Configuration) -> ApiResult<T>,
        T: Serialize + Deserialize<'a>,
    {
        let cache_entry: &mut CacheEntry = match self.cache.entry(endpoint) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let ce = CacheEntry {
                    datetime: Local::now(),
                    response_json: serde_json::to_string(&api_call(&self.config)?)?,
                };
                v.insert(ce)
            }
        };

        if Local::now() - cache_entry.datetime < self.refresh_duration && !self.force_refresh {
            self.cache_hit += 1;
            return Ok(serde_json::from_str(&cache_entry.response_json)?);
        }

        let resp = api_call(&self.config)?;

        cache_entry.response_json = serde_json::to_string(&resp)?;

        Ok(resp)
    }

    pub fn get_budgets(&mut self) -> ApiResult<Vec<BudgetSummary>> {
        let endp = "/budgets".to_string();
        let response = self.get(endp, |config| Ok(budgets_api::get_budgets(config, None)?))?;

        Ok(response.data.budgets)
    }

    pub fn get_accounts(&mut self, budget_id: &str) -> ApiResult<Vec<Account>> {
        let endp = format!("/budgets/{budget_id}/accounts");
        let response = self.get(endp, |config| {
            Ok(accounts_api::get_accounts(config, budget_id, None)?)
        })?;

        Ok(response.data.accounts)
    }

    pub fn get_transactions_by_account(
        &mut self,
        budget_id: &str,
        account_id: &str,
    ) -> ApiResult<Vec<TransactionDetail>> {
        let endp = format!("/budgets/{budget_id}/accounts/{account_id}/transactions");
        let response = self.get(endp, |config| {
            Ok(transactions_api::get_transactions_by_account(
                config, budget_id, account_id, None, None, None,
            )?)
        })?;

        let mut ts = response.data.transactions;

        ts.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(ts)
    }

    pub fn get_transactions(&mut self, budget_id: &str) -> ApiResult<Vec<TransactionDetail>> {
        let endp = format!("/budgets/{budget_id}/transactions");

        let response = self.get(endp, |config| {
            Ok(transactions_api::get_transactions(
                config, budget_id, None, None, None,
            )?)
        })?;
        let mut ts = response.data.transactions;
        ts.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(ts)
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
