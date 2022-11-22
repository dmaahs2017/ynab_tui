use super::models::*;
use sqlite::{Result, Statement };
use sqlite::{self, Connection, State};

trait CollectQuery<T> 
    where T: ReadFromStatement
{
    fn collect(&mut self) -> Result<Vec<T>>;
}

impl<'a, T> CollectQuery<T> for Statement<'a> 
    where T: ReadFromStatement
{
    fn collect(&mut self) -> Result<Vec<T>> {
        let mut output = vec![];
        while let Ok(State::Row) = self.next() {
            output.push(T::read(self)?);
        }
        return Ok(output);
    }
}

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
        statement
            .bind((":id", budget_id))
            .expect("Failed to bind prepared statement");
        if let Ok(State::Row) = statement.next() {
            return Some(Budget::read(&mut statement).expect("Failed to read statement"));
        }
        return None;
    }

    pub fn get_all_budgets(&self) -> Vec<Budget> {
        let query = include_str!("queries/get_all_budgets.sql");
        let mut statement = self.conn.prepare(query).expect("Prepared select failed");
        statement.collect().expect("Failed to collect budgets")
    }

    pub fn insert_budget(&self, budget: Budget) {
        let query = include_str!("queries/insert_budget.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        budget.bind(&mut statement).expect("Failed to bind budget");
        statement.next().expect("Insert failed");
    }

    pub fn update_budget(&self, budget: Budget) {
        let query = include_str!("queries/update_budget.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        budget.bind(&mut statement).expect("Failed to bind budget");
        statement.next().expect("Insert failed");
    }

    pub fn insert_transaction(&self, transaction: Transaction) -> Result<()> {
        let query = include_str!("queries/insert_transaction.sql");
        let mut statement = self.conn.prepare(query)?;
        transaction.bind(&mut statement)?;
        statement.next()?;
        Ok(())
    }

    pub fn get_transaction(&self, transaction_id: &str) -> Option<Transaction> {
        let query = include_str!("queries/get_transaction_by_id.sql");
        let mut statement = self.conn.prepare(query).expect("Prepared select failed");
        statement
            .bind((":id", transaction_id))
            .expect("Failed to bind id");

        if let Ok(State::Row) = statement.next() {
            return Some(Transaction::read(&mut statement).expect("Failed to read transaction"));
        }
        return None;
    }

    pub fn get_transactions_where(
        &self,
        budget_id: &str,
        search_query: &str,
    ) -> Result<Vec<Transaction>> {
        let query = format!(
            include_str!("queries/get_transactions_where.sql"),
            search_query
        );
        let mut statement = self.conn.prepare(query)?;
        statement.bind_iter([(":budget_id", budget_id)])?;
        statement.collect()
    }

    pub fn get_transactions(&self, budget_id: &str) -> Vec<Transaction> {
        let query = include_str!("queries/get_all_transactions.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        statement
            .bind((":budget_id", budget_id))
            .expect("Failed to bind prepared statement");

        statement.collect().expect("Failed to collect transactions")
    }

    pub fn update_transaction(&self, transaction: Transaction) -> Result<()> {
        let query = include_str!("queries/update_transaction.sql");
        let mut statement = self.conn.prepare(query)?;
        transaction.bind(&mut statement)?;
        statement.next()?;
        Ok(())
    }

    pub fn remigrate(&self) {
        let down_query = include_str!("migrations/down.sql");
        let up_query = include_str!("migrations/up.sql");
        self.conn.execute(down_query).expect("Drop all failed");
        self.conn.execute(up_query).expect("Drop all failed");
    }
}

fn setup(conn: &Connection) {
    let setup_query = include_str!("migrations/up.sql");
    conn.execute(setup_query).expect("Setup query failed");
}
