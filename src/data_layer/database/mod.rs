use super::models::*;
use sqlite::{self, Connection, State};
use sqlite::{Result, Statement};

trait CollectQuery<T>
where
    T: ReadFromStatement,
{
    fn collect(&mut self) -> Result<Vec<T>>;
}

impl<'a, T> CollectQuery<T> for Statement<'a>
where
    T: ReadFromStatement,
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

    pub fn select_all<T: AllSelectable + ReadFromStatement>(&self) -> Result<Vec<T>> {
        let mut statement = self.conn.prepare(T::query())?;
        statement.collect()
    }

    pub fn insert<T: Insertable + BindToStatement>(&self, a: T) -> Result<()> {
        let mut statement = self.conn.prepare(a.query())?;
        a.bind(&mut statement)?;
        statement.next()?;
        Ok(())
    }

    pub fn update<T: Updateable + BindToStatement>(&self, a: T) -> Result<()> {
        let mut statement = self.conn.prepare(a.query())?;
        a.bind(&mut statement)?;
        statement.next()?;
        Ok(())
    }

    pub fn select_by_id<T: IdSelectable + ReadFromStatement>(&self, id: &str) -> Result<Option<T>> {
        let mut statement = self.conn.prepare(T::query())?;
        statement.bind((":id", id))?;
        if let Ok(State::Row) = statement.next() {
            return Ok(Some(T::read(&mut statement)?));
        }
        return Ok(None);
    }

    pub fn get_transactions_where(
        &self,
        budget_id: &str,
        search_query: &str,
    ) -> Result<Vec<Transaction>> {
        let query = format!(
            include_str!("queries/transaction/select_where.sql"),
            search_query
        );
        let mut statement = self.conn.prepare(query)?;
        statement.bind_iter([(":budget_id", budget_id)])?;
        statement.collect()
    }

    pub fn get_transactions(&self, budget_id: &str) -> Result<Vec<Transaction>> {
        let query = include_str!("queries/transaction/select_all.sql");
        let mut statement = self.conn.prepare(query)?;
        statement.bind((":budget_id", budget_id))?;
        statement.collect()
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
