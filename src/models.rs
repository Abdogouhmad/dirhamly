use chrono::NaiveDate;
use rust_decimal::Decimal;

// Note: You might need to import TransactionType and Category from your commands/cli file
// or move those enums here so both the DB and CLI can use them.
use crate::commands::{Category, TransactionType};

#[derive(Debug)]
pub struct Transaction {
    /// The unique ID of the transaction (None until inserted into the DB)
    pub id: Option<i64>,
    pub tx_type: TransactionType,
    pub amount: Decimal,
    pub category: Category,
    pub description: String,
    pub date: NaiveDate,
}

impl Transaction {
    pub fn new(
        tx_type: TransactionType,
        amount: Decimal,
        category: Category,
        description: String,
        date: NaiveDate,
    ) -> Self {
        Self { id: None, tx_type, amount, category, description, date }
    }
}
