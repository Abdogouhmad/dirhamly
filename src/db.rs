use std::str::FromStr;

use crate::{
    commands::{RemoveArgs, TransactionType},
    models::Transaction,
};
use chrono::NaiveDate;
// use chrono::NaiveDate;
use rusqlite::{Connection, Result, Row, params};
use rust_decimal::Decimal;
// use rust_decimal::Decimal;
// use std::str::FromStr;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY,
                tx_type TEXT NOT NULL,
                amount TEXT NOT NULL,
                category TEXT NOT NULL,
                description TEXT,
                date TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_transaction(&self, transaction: &Transaction) -> Result<()> {
        self.conn.execute(
            "INSERT INTO transactions (tx_type, amount, category, description, date) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                transaction.tx_type.to_string(),
                transaction.amount.to_string(),
                transaction.category.to_string(),
                transaction.description,
                transaction.date.to_string()
            ],
        )?;
        Ok(())
    }
    // for viewing transactions, we can implement a method like this:
    // Retrieve transactions filtered by optional criteria
    /// Retrieve transactions with optional filters.
    /// - `from` / `to`: date range (inclusive)
    /// - `tx_type`: optional transaction type filter
    /// - `category`: optional category filter (pass `None` or empty string to ignore)
    pub fn get_transactions(
        &self,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        tx_type: Option<TransactionType>,
        category: Option<String>, // ← changed to Option<String>
    ) -> Result<Vec<Transaction>> {
        let mut query = String::from(
            "SELECT id, tx_type, amount, category, description, date 
             FROM transactions 
             WHERE 1=1",
        );

        let from_str = from.map(|f| f.to_string());
        let to_str = to.map(|t| t.to_string());
        let tx_type_str = tx_type.map(|t| t.to_string());

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(f) = &from_str {
            query.push_str(" AND date >= ?");
            params.push(f);
        }

        if let Some(t) = &to_str {
            query.push_str(" AND date <= ?");
            params.push(t);
        }

        if let Some(t_type) = &tx_type_str {
            query.push_str(" AND tx_type = ?");
            params.push(t_type);
        }

        if let Some(cat) = &category {
            if !cat.is_empty() {
                query.push_str(" AND category = ?");
                params.push(cat);
            }
        }

        // Almost always useful: sort by date descending
        query.push_str(" ORDER BY date DESC");

        let mut stmt = self.conn.prepare(&query)?;

        let transaction_iter = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Self::map_row_to_transaction(row)
        })?;

        let mut transactions = Vec::new();
        for result in transaction_iter {
            transactions.push(result?);
        }

        Ok(transactions)
    }

    /// Helper to map a database row → Transaction struct
    /// Returns error instead of panicking if parsing fails
    fn map_row_to_transaction(row: &Row<'_>) -> rusqlite::Result<Transaction> {
        let id: i64 = row.get(0)?;
        let tx_type_str: String = row.get(1)?;
        let amount_str: String = row.get(2)?;
        let category: String = row.get(3)?;
        let description: String = row.get(4)?;
        let date_str: String = row.get(5)?;

        let tx_type = TransactionType::from_str(&tx_type_str)
            .map_err(|e| rusqlite::Error::InvalidParameterName(format!("Invalid tx_type: {e}")))?;

        let amount = Decimal::from_str(&amount_str)
            .map_err(|e| rusqlite::Error::InvalidParameterName(format!("Invalid amount: {e}")))?;

        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
            rusqlite::Error::InvalidParameterName(format!("Invalid date format: {e}"))
        })?;

        Ok(Transaction { id: Some(id), tx_type, amount, category, description, date })
    }

    /// fn that remove the data from the database based on the id
    pub fn delete_by_id(&self, target: &RemoveArgs) -> Result<usize, rusqlite::Error> {
        self.conn.execute("DELETE FROM transactions WHERE id = ?1", params![target.id])
    }
}
