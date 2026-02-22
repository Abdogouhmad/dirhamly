use crate::{commands::{Category, TransactionType}, models::Transaction};
use chrono::NaiveDate;
use rusqlite::{Connection, Result, params};
use rust_decimal::Decimal;
use std::str::FromStr;

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
    /// Retrieve transactions filtered by optional criteria
    pub fn get_transactions(
        &self,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
        tx_type: Option<TransactionType>,
        category: Option<Category>,
    ) -> Result<Vec<Transaction>> {
        // 1. Build the SQL query dynamically based on what filters are provided
        let mut query = "SELECT id, tx_type, amount, category, description, date FROM transactions WHERE 1=1".to_string();
        let mut params: Vec<String> = Vec::new();

        if let Some(f) = from {
            query.push_str(&format!(" AND date >= ?{}", params.len() + 1));
            params.push(f.to_string());
        }
        if let Some(t) = to {
            query.push_str(&format!(" AND date <= ?{}", params.len() + 1));
            params.push(t.to_string());
        }
        if let Some(t_type) = tx_type {
            query.push_str(&format!(" AND tx_type = ?{}", params.len() + 1));
            params.push(t_type.to_string());
        }
        if let Some(cat) = category {
            query.push_str(&format!(" AND category = ?{}", params.len() + 1));
            params.push(cat.to_string());
        }

        query.push_str(" ORDER BY date DESC");

        // 2. Prepare and execute the query
        let mut stmt = self.conn.prepare(&query)?;

        // 3. Map the database rows back into Rust `Transaction` structs
        let transaction_iter = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            // Get raw strings from the database
            let tx_type_str: String = row.get(1)?;
            let amount_str: String = row.get(2)?;
            let category_str: String = row.get(3)?;
            let description: String = row.get(4)?;
            let date_str: String = row.get(5)?;

            // Parse strings back into Rust types. 
            // If parsing fails, we return a database error.
            let tx_type = TransactionType::from_str(&tx_type_str)
                .map_err(|_| rusqlite::Error::InvalidColumnType(1, "tx_type".to_string(), rusqlite::types::Type::Text))?;
            
            let amount = Decimal::from_str(&amount_str)
                .map_err(|_| rusqlite::Error::InvalidColumnType(2, "amount".to_string(), rusqlite::types::Type::Text))?;
                
            let category = Category::from_str(&category_str)
                .map_err(|_| rusqlite::Error::InvalidColumnType(3, "category".to_string(), rusqlite::types::Type::Text))?;
                
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|_| rusqlite::Error::InvalidColumnType(5, "date".to_string(), rusqlite::types::Type::Text))?;

            Ok(Transaction {
                id: row.get(0)?,
                tx_type,
                amount,
                category,
                description,
                date,
            })
        })?;

        // Collect the iterator into a Vec
        let mut transactions = Vec::new();
        for tx in transaction_iter {
            transactions.push(tx?);
        }

        Ok(transactions)
    }
}
