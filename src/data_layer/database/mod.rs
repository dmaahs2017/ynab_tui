use super::models::*;
use sqlite::{self, Connection, Result, State, Statement, Value as SqlValue};

trait IntoSqlValue {
    fn into_sql_value(self) -> SqlValue;
}

impl IntoSqlValue for Option<String> {
    fn into_sql_value(self) -> SqlValue {
        match self {
            None => SqlValue::Null,
            Some(s) => SqlValue::String(s),
        }
    }
}

impl IntoSqlValue for bool {
    fn into_sql_value(self) -> SqlValue {
        if self {
            SqlValue::Integer(1)
        } else {
            SqlValue::Integer(0)
        }
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
            return Some(read_budget_from_statement(&mut statement));
        }
        None
    }

    pub fn get_all_budgets(&self) -> Vec<Budget> {
        let query = include_str!("queries/get_all_budgets.sql");
        let mut statement = self.conn.prepare(query).expect("Prepared select failed");
        let mut output = vec![];
        while let Ok(State::Row) = statement.next() {
            output.push(read_budget_from_statement(&mut statement));
        }
        output
    }

    pub fn insert_budget(&self, budget: Budget) {
        let query = include_str!("queries/insert_budget.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        statement
            .bind_iter::<_, (_, SqlValue)>([
                (":id", budget.id.into()),
                (":name", budget.name.into()),
                (":last_modified_on", budget.last_modified_on.into()),
                (":first_month", budget.first_month.into()),
                (":last_month", budget.last_month.into()),
                (":date_format", budget.date_format.into()),
            ])
            .expect("Insert failed");
        statement.next().expect("Insert failed");
    }

    pub fn update_budget(&self, budget: Budget) {
        let query = include_str!("queries/update_budget.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        statement
            .bind_iter::<_, (_, SqlValue)>([
                (":id", budget.id.into()),
                (":name", budget.name.into()),
                (":last_modified_on", budget.last_modified_on.into()),
                (":first_month", budget.first_month.into()),
                (":last_month", budget.last_month.into()),
                (":date_format", budget.date_format.into()),
            ])
            .expect("Insert failed");
        statement.next().expect("Insert failed");
    }

    pub fn insert_transaction(&self, transaction: Transaction) -> Result<()> {
        let query = include_str!("queries/insert_transaction.sql");
        let mut statement = self.conn.prepare(query)?;
        statement.bind_iter::<_, (_, SqlValue)>([
            (":id", transaction.id.into()),
            (":budget_id", transaction.budget_id.into()),
            (":date", transaction.date.into()),
            (":amount", transaction.amount.into()),
            (":memo", transaction.memo.into_sql_value()),
            (":account_id", transaction.account_id.into()),
            (":payee_id", transaction.payee_id.into_sql_value()),
            (":category_id", transaction.category_id.into_sql_value()),
            (
                ":transfer_account_id",
                transaction.transfer_account_id.into_sql_value(),
            ),
            (
                ":transfer_transaction_id",
                transaction.transfer_transaction_id.into_sql_value(),
            ),
            (
                ":matched_transaction_id",
                transaction.matched_transaction_id.into_sql_value(),
            ),
            (":account_name", transaction.account_name.into()),
            (":payee_name", transaction.payee_name.into_sql_value()),
            (":category_name", transaction.category_name.into()),
        ])?;
        statement.next()?;
        Ok(())
    }

    pub fn get_transaction(&self, transaction_id: &str) -> Option<Transaction> {
        let query = include_str!("queries/get_transaction_by_id.sql");
        let mut statement = self.conn.prepare(query).expect("Prepared select failed");
        statement
            .bind((":id", transaction_id))
            .expect("Failed to bind prepared statement");

        if let Ok(State::Row) = statement.next() {
            return Some(read_transaction_from_statement(&mut statement));
        }
        None
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

        let mut output = vec![];
        while let Ok(State::Row) = statement.next() {
            output.push(read_transaction_from_statement(&mut statement));
        }
        Ok(output)
    }

    pub fn get_transactions(&self, budget_id: &str) -> Vec<Transaction> {
        let query = include_str!("queries/get_all_transactions.sql");
        let mut statement = self.conn.prepare(query).expect("Insert failed");
        statement
            .bind((":budget_id", budget_id))
            .expect("Failed to bind prepared statement");

        let mut output = vec![];
        while let Ok(State::Row) = statement.next() {
            output.push(read_transaction_from_statement(&mut statement));
        }
        output
    }

    pub fn update_transaction(&self, transaction: Transaction) -> Result<()> {
        let query = include_str!("queries/update_transaction.sql");
        let mut statement = self.conn.prepare(query)?;
        statement.bind_iter([
            (":id", transaction.id.into()),
            (":date", transaction.date.into()),
            (":amount", transaction.amount.into()),
            (":memo", transaction.memo.into_sql_value()),
            (":account_id", transaction.account_id.into()),
            (":payee_id", transaction.payee_id.into_sql_value()),
            (":category_id", transaction.category_id.into_sql_value()),
            (
                ":transfer_account_id",
                transaction.transfer_account_id.into_sql_value(),
            ),
            (
                ":transfer_transaction_id",
                transaction.transfer_transaction_id.into_sql_value(),
            ),
            (
                ":matched_transaction_id",
                transaction.matched_transaction_id.into_sql_value(),
            ),
            (":account_name", transaction.account_name.into()),
            (":payee_name", transaction.payee_name.into_sql_value()),
            (":category_name", transaction.category_name.into()),
        ])?;
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

fn read_transaction_from_statement(statement: &mut Statement) -> Transaction {
    Transaction {
        id: statement.read("id").unwrap(),
        budget_id: statement.read("budget_id").unwrap(),
        date: statement.read("date").unwrap(),
        amount: statement.read("amount").unwrap(),
        memo: statement.read("memo").ok(),
        account_id: statement.read("account_id").unwrap(),
        payee_id: statement.read("payee_id").ok(),
        category_id: statement.read("category_id").ok(),
        transfer_account_id: statement.read("transfer_account_id").ok(),
        transfer_transaction_id: statement.read("transfer_transaction_id").ok(),
        matched_transaction_id: statement.read("matched_transaction_id").ok(),
        account_name: statement.read("account_name").unwrap(),
        payee_name: statement.read("payee_name").ok(),
        category_name: statement.read("category_name").unwrap(),
    }
}

fn read_budget_from_statement(statement: &mut Statement) -> Budget {
    Budget {
        id: statement.read("id").unwrap(),
        name: statement.read("name").unwrap(),
        last_modified_on: statement.read("last_modified_on").unwrap(),
        first_month: statement.read("first_month").unwrap(),
        last_month: statement.read("last_month").unwrap(),
        date_format: statement.read("date_format").unwrap(),
    }
}
