use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use rust_decimal::Decimal;
use strum::EnumString;
use strum_macros::{Display, VariantNames};
use crate::{models::Transaction, utils::format_list};

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
    propagate_version = true
)]
pub struct DirhamlyCli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new transaction (income or expense)
    Add(AddArgs),

    /// List all transactions
    List(ListArgs),

    /// Generate a summary report
    Summary(SummaryArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct AddArgs {
    /// Type of transaction (income or expense)
    #[arg(value_enum)]
    pub tx_type: TransactionType,

    /// Amount of the transaction (e.g., 25.50)
    pub amount: Decimal,

    /// Category of the transaction
    #[arg(value_enum)]
    pub category: Category,

    /// Description of the transaction
    #[arg(short, long)]
    pub description: String,

    /// Date of the transaction (YYYY-MM-DD). Defaults to today if omitted.
    #[arg(long)]
    pub date: Option<NaiveDate>,
}

#[derive(Parser, Debug, PartialEq)]
pub struct ListArgs {
    /// Filter by transaction type
    #[arg(short, long, value_enum)]
    pub tx_type: Option<TransactionType>,

    /// Filter by category
    #[arg(short, long, value_enum)]
    pub category: Option<Category>,

    /// Filter from date (YYYY-MM-DD)
    #[arg(long)]
    pub from: Option<NaiveDate>,

    /// Filter to date (YYYY-MM-DD)
    #[arg(long)]
    pub to: Option<NaiveDate>,
}

#[derive(Parser, Debug, PartialEq)]
pub struct SummaryArgs {
    /// Summarize by period
    #[arg(short, long, value_enum)]
    pub period: Option<Period>,

    // /// Filter by category
    #[arg(short, long, value_enum)]
    pub category: Option<Category>,
}

// --- ENUMS ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Display, EnumString, VariantNames)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, EnumString, Display, VariantNames)]
pub enum Category {
    // Expense Categories
    Food,
    Transport,
    Utilities,
    Entertainment,
    Health,
    Withdrawals,
    // Income Categories
    Salary,
    Gift,
    Investment,
    // General
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, EnumString, Display, VariantNames)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl DirhamlyCli {
    /// Executes the parsed command by matching on the command type
    pub fn run(&self, db: &crate::db::Database) {
        println!("--- Executing Command ---");
        match &self.command {
            Command::Add(args) => {
                // If date is None, fallback to today's local date
                let date = args.date.unwrap_or_else(|| Local::now().date_naive());

                // Construct the Transaction model
                let transaction = Transaction::new(
                    args.tx_type,
                    args.amount,
                    args.category,
                    args.description.clone(),
                    date,
                );

                // Insert into the database
                match db.add_transaction(&transaction) {
                    Ok(_) => {
                        println!(
                            "✅ Added {} of {} MAD for {} on {}",
                            transaction.tx_type,
                            transaction.amount,
                            transaction.category,
                            transaction.date
                        );
                    }
                    Err(e) => eprintln!("❌ Failed to save transaction: {}", e),
                }
            }
            // NOTE: bring from database and filter by the provided criteria (date range, type of transaction, category)
            Command::List(args) => {
                println!("--- Transactions ---");
                
                // Call the get_transactions method from db.rs
                match db.get_transactions(args.from, args.to, args.tx_type, args.category) {
                    Ok(transactions) => {
                        if transactions.is_empty() {
                            println!("No transactions found matching your criteria.");
                        } else {
                            // Loop through and display each transaction
                            format_list(&transactions);
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to retrieve transactions: {}", e),
                }
            }
            // NOTE: bring from database and summarize by the provided criteria (period, category)
            // and calculate: Food - 1000 (10% of total expenses), Salary - 5000 (50% of total income), etc.
            Command::Summary(args) => {
                println!("Action: Generate Summary");
                if let Some(period) = &args.period {
                    println!("Period: {}", period);
                }
                if let Some(category) = &args.category {
                    println!("Filter Category: {}", category);
                }
            }
        }
    }
}
