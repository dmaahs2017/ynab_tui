use std::env;
use sqlite::{self, Connection, State};
use super::models::*;

pub struct QueryEngine {
    conn: Connection,
}

impl QueryEngine {
    pub fn new(db_url: &str) -> Self {
        let conn = sqlite::open(db_url).expect("Connection opened");
        setup(&conn);
        Self { conn }
    }

    pub fn get_budget(&self, budget_id: &str) -> Option<Budget> {
        let query = include_str!("queries/get_budget_by_id.sql");
        let mut statement = self.conn.prepare(query).expect("Prepared select failed");
        statement.bind((":id", budget_id)).expect("Failed to bind prepared statement");
        if let Ok(State::Row) = statement.next() {
            return Some(Budget {
                id: statement.read("id").unwrap(),
                name: statement.read("name").unwrap(),
                last_modified_on: statement.read("last_modified_on").unwrap(),
                first_month: statement.read("first_month").unwrap(),
                last_month: statement.read("last_month").unwrap(),
                date_format: statement.read("date_format").unwrap(),
            })
        }
        return None
    }

    pub fn get_all_budgets(&self) -> Vec<Budget> {
        let query = include_str!("queries/get_all_budgets.sql");
        let mut statement = self.conn.prepare(query).expect("Prepared select failed");
        let mut output = vec![];
        while let Ok(State::Row) = statement.next() {
            output.push(Budget {
                id: statement.read("id").unwrap(),
                name: statement.read("name").unwrap(),
                last_modified_on: statement.read("last_modified_on").unwrap(),
                first_month: statement.read("first_month").unwrap(),
                last_month: statement.read("last_month").unwrap(),
                date_format: statement.read("date_format").unwrap(),
            });
        }
        return output
    }

    pub fn insert_budget(&self, budget: Budget) {
        let query = include_str!("queries/insert_budget.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        statement.bind_iter([
            (":id", budget.id.as_str()),
            (":name", budget.name.as_str()),
            (":last_modified_on", budget.last_modified_on.as_str()),
            (":first_month", budget.first_month.as_str()),
            (":last_month", budget.last_month.as_str()),
            (":date_format", budget.date_format.as_str()),
        ]).expect("Insert failed");
        statement.next().expect("Insert failed");
    }

    pub fn update_budget(&self, budget: Budget) {
        let query = include_str!("queries/update_budget.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        statement.bind_iter([
            (":id", budget.id.as_str()),
            (":name", budget.name.as_str()),
            (":last_modified_on", budget.last_modified_on.as_str()),
            (":first_month", budget.first_month.as_str()),
            (":last_month", budget.last_month.as_str()),
            (":date_format", budget.date_format.as_str()),
        ]).expect("Insert failed");
        statement.next().expect("Insert failed");
    }

}

fn setup(conn: &Connection) {
    let setup_query = include_str!("migrations/up.sql");
    conn.execute(setup_query).expect("Setup query failed");
}

pub fn tear_down() {
    let database_url = env::var("DATABASE_URL").expect("Database url environment variable not set");
    let conn = sqlite::open(database_url).expect("Connection opened");
    let down_query = include_str!("migrations/down.sql");
    println!("{}", down_query);
    conn.execute(down_query).expect("Setup query failed");
}
