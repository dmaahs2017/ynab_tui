mod ynab_api;
mod database;
pub mod models;

use ynab_api::YnabApi;
use database::QueryEngine;

use chrono::Duration;
use std::env;

pub struct DataGateway {
    api: YnabApi,
    engine: QueryEngine,
}

impl DataGateway {
    pub fn new() -> Self {
        let token = env::var("YNAB_TOKEN").expect("Ynab token not in env");
        let cache_file = env::var("YNAB_CACHE_FILE").expect("YNAB_CACHE_FILE not in env");
        let mut api = ynab_api::YnabApi::new(&token, &cache_file, Duration::hours(1));
        let engine = QueryEngine::new(&std::env::var("DATABASE_URL").expect("DATABASE_URL not in env"));
        Self { api, engine }
    }

    pub async fn load_budgets(&mut self) {
        let budget_list = self.api.list_budgets(false).await.expect("Failed to get budgets from api");
        for b in budget_list.data.budgets {
            if let Some(_) = self.engine.get_budget(&b.id) {
                self.engine.update_budget(models::Budget {
                    id: b.id,
                    name: b.name,
                    last_modified_on: b.last_modified_on,
                    first_month: b.first_month,
                    last_month: b.last_month,
                    date_format: b.date_format.format
                })
            } else {
                self.engine.insert_budget(models::Budget {
                    id: b.id,
                    name: b.name,
                    last_modified_on: b.last_modified_on,
                    first_month: b.first_month,
                    last_month: b.last_month,
                    date_format: b.date_format.format
                })
            }
        }
    }

    pub fn get_budgets(&self) -> Vec<models::Budget> {
        self.engine.get_all_budgets()
    }
}

