use std::collections::HashMap;

use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::commands::TransactionType;
use crate::models::Transaction;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Report {
    pub total_income: Decimal,
    pub total_expenses: Decimal,
    pub balance: Decimal,
    pub transaction_count: usize,
    pub transactions: Vec<ReportTransaction>,
    pub category_breakdown: Vec<CategorySummary>,
}

#[derive(Debug, Serialize)]
pub struct ReportTransaction {
    pub id: i64,
    pub tx_type: String, // "Income" or "Expense"
    pub category: String,
    pub description: String,
    pub amount: Decimal,
    pub date: String, // "2025-04-12"
}

#[derive(Debug, Serialize)]
pub struct CategorySummary {
    pub category: String,
    pub amount: Decimal,
    pub percentage: f64, // 0.0 .. 100.0
}

pub fn generate_report(transactions: &[Transaction]) -> Report {
    if transactions.is_empty() {
        return Report {
            total_income: Decimal::ZERO,
            total_expenses: Decimal::ZERO,
            balance: Decimal::ZERO,
            transaction_count: 0,
            transactions: vec![],
            category_breakdown: vec![],
        };
    }

    let mut income = Decimal::ZERO;
    let mut expenses = Decimal::ZERO;
    let mut by_category: HashMap<String, Decimal> = HashMap::new();

    let mut report_tx = Vec::with_capacity(transactions.len());

    for tx in transactions {
        let id = match tx.id {
            Some(i) => i,
            None => {
                eprintln!("Warning: skipping transaction without ID: {:?}", tx);
                continue; // or you can use -1 or some sentinel value
            }
        };

        report_tx.push(ReportTransaction {
            id,
            tx_type: tx.tx_type.to_string(),
            category: tx.category.clone(),
            description: tx.description.clone(),
            amount: tx.amount,
            date: tx.date.to_string(),
        });

        match tx.tx_type {
            TransactionType::Income => income += tx.amount,
            TransactionType::Expense => {
                expenses += tx.amount;
                *by_category.entry(tx.category.clone()).or_insert(Decimal::ZERO) += tx.amount;
            }
        }
    }

    let balance = income - expenses;

    // Category breakdown — only for expenses
    let mut breakdown: Vec<CategorySummary> = if expenses.is_zero() {
        // No expenses → empty or single 0% entry if you want
        vec![]
    } else {
        by_category
            .into_iter()
            .map(|(category, amount)| {
                let percentage = ((amount / expenses) * dec!(100)).to_f64().unwrap_or(0.0);
                CategorySummary { category, amount, percentage }
            })
            .collect()
    };

    // Sort descending by amount
    breakdown.sort_by(|a, b| b.amount.cmp(&a.amount));

    Report {
        total_income: income,
        total_expenses: expenses,
        balance,
        transaction_count: report_tx.len(), // use filtered count
        transactions: report_tx,
        category_breakdown: breakdown,
    }
}
