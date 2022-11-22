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
        let budgets = self.get_budgets();
        for b in budgets {
            self.load_budget_export(&b.id)?;
        }
        Ok(())
    }

    fn load_budget_export(&mut self, budget_id: &str) -> Result<()> {
        let export = self.api.budget_export(budget_id, None).expect("Failed to get budget export from api").data.budget;
        for account in export.accounts {
            self.engine.insert_account(account.into())
        }
        Ok(())
    }

    fn load_budgets(&mut self) {
        let budget_list = self
            .api
            .list_budgets(false)
            .expect("Failed to get budgets from api");
        for b in budget_list.data.budgets {
            let b: Budget = b.into();
            if let Some(_) = self.engine.get_budget(&b.id) {
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

        for t in transactions.data.transactions {
            let t = Transaction::from_detail(t, budget_id);
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
