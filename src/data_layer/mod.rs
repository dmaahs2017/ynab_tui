mod database;
pub mod models;
mod ynab_api;
pub use models::*;

use database::QueryEngine;
use ynab_api::YnabApi;

use chrono::Duration;
use sqlite::Result;
use std::env;

pub struct DataGateway {
    api: YnabApi,
    engine: QueryEngine,
}

#[rustfmt::skip]
impl Default for DataGateway { fn default() -> Self { Self::new() } }

impl DataGateway {
    pub fn new() -> Self {
        let token = env::var("YNAB_TOKEN").expect("Ynab token not in env");
        let cache_file = env::var("YNAB_CACHE_FILE").expect("YNAB_CACHE_FILE not in env");
        let api = ynab_api::YnabApi::new(&token, &cache_file, Duration::seconds(20));
        let engine =
            QueryEngine::new(&std::env::var("DATABASE_URL").expect("DATABASE_URL not in env"));
        Self { api, engine }
    }

    pub fn refresh_db(&mut self) -> Result<()> {
        self.engine.remigrate();
        self.load_budgets();
        let budgets = self
            .get_budgets()
            .iter()
            .map(|b| b.id.to_string())
            .collect::<Vec<_>>();
        for budget_id in budgets {
            self.load_transactions(&budget_id)?;
        }
        Ok(())
    }

    fn load_budgets(&mut self) {
        let budget_list = self
            .api
            .list_budgets(false)
            .expect("Failed to get budgets from api");
        for b in budget_list["data"]["budgets"]
            .as_array()
            .expect(".data.budgets is a list of Budgets")
        {
            let b = models::Budget {
                id: b["id"].as_str().unwrap().to_string(),
                name: b["name"].as_str().unwrap().to_string(),
                last_modified_on: b["last_modified_on"].as_str().unwrap().to_string(),
                first_month: b["first_month"].as_str().unwrap().to_string(),
                last_month: b["last_month"].as_str().unwrap().to_string(),
                date_format: b["date_format"]["format"].as_str().unwrap().to_string(),
            };

            if self.engine.get_budget(&b.id).is_some() {
                self.engine.update_budget(b)
            } else {
                self.engine.insert_budget(b)
            }
        }
    }

    pub fn get_budgets(&self) -> Vec<Budget> {
        self.engine.get_all_budgets()
    }

    fn load_transactions(&mut self, budget_id: &str) -> Result<()> {
        let transactions = self
            .api
            .get_budget_transactions(budget_id, None, None, None)
            .expect("Failed to get transactions from api");

        //for t in transactions.data.transactions {
        for t in transactions["data"]["transactions"]
            .as_array()
            .expect(".data.transcations is an array of transactions")
        {
            let t = Transaction {
                id: t["id"].as_str().unwrap().to_string(),
                budget_id: budget_id.to_string(),
                date: t["date"].as_str().unwrap().to_string(),
                amount: t["amount"].as_i64().unwrap(),
                memo: t["memo"].as_str().map(str::to_string),
                account_id: t["account_id"].as_str().unwrap().to_string(),
                payee_id: t["payee_id"].as_str().map(str::to_string),
                category_id: t["category_id"].as_str().map(str::to_string),
                transfer_account_id: t["transfer_account_id"].as_str().map(str::to_string),
                transfer_transaction_id: t["transfer_transaction_id"].as_str().map(str::to_string),
                matched_transaction_id: t["matched_transaction_id"].as_str().map(str::to_string),
                account_name: t["account_name"].as_str().unwrap().to_string(),
                payee_name: t["payee_name"].as_str().map(str::to_string),
                category_name: t["category_name"].as_str().unwrap().to_string(),
            };
            if let Some(db_transaction) = self.engine.get_transaction(&t.id) {
                if db_transaction != t {
                    self.engine.update_transaction(t)?;
                }
            } else {
                self.engine.insert_transaction(t)?;
            }
        }
        Ok(())
    }

    pub fn get_transactions(&self, budget_id: &str) -> Vec<Transaction> {
        self.engine.get_transactions(budget_id)
    }

    pub fn get_transactions_where(&self, budget_id: &str, query: &str) -> Result<Vec<Transaction>> {
        if query.is_empty() {
            return self.engine.get_transactions_where(budget_id, "0 = 0");
        }
        self.engine.get_transactions_where(budget_id, query)
    }
}
