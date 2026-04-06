use anyhow::Result;
use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use rust_decimal::Decimal;
use strum_macros::{Display, EnumString, VariantNames};

use crate::db::Database;
use crate::models::Transaction;
use crate::utils::{export_transactions, format_list};

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct DirhamlyCli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new income or expense transaction
    #[command(about = "Add a new income or expense transaction")]
    Add(AddArgs),

    /// List transactions (with optional filters)
    #[command(about = "Display transactions in a formatted table")]
    Read(ReadArgs),

    /// Export transactions to CSV or PDF report
    #[command(about = "Export filtered transactions to CSV or styled PDF")]
    Export(ExportArgs),

    /// Remov the operation by its id
    Remove(RemoveArgs),
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    /// Transaction type (income or expense)
    #[arg(value_enum)]
    pub tx_type: TransactionType,

    /// Amount (e.g. 250.75)
    pub amount: Decimal,

    /// Category (e.g. Salary, Food, Rent)
    #[arg(short, long)]
    pub category: String,

    /// Optional description
    #[arg(short, long)]
    pub description: String,

    /// Transaction date (YYYY-MM-DD). Defaults to today.
    #[arg(long)]
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, ValueEnum, Display, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Parser, Debug)]
pub struct ReadArgs {
    /// Start date filter (YYYY-MM-DD)
    #[arg(long)]
    pub from: Option<NaiveDate>,

    /// End date filter (YYYY-MM-DD)
    #[arg(long)]
    pub to: Option<NaiveDate>,

    /// Filter by transaction type
    #[arg(long, value_enum)]
    pub tx_type: Option<TransactionType>,

    /// Filter by category
    #[arg(short, long)]
    pub category: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ExportArgs {
    /// Output format
    #[arg(value_enum)]
    pub format: ExportFormat,

    /// Start date filter (YYYY-MM-DD)
    #[arg(long)]
    pub from: Option<NaiveDate>,

    /// End date filter (YYYY-MM-DD)
    #[arg(long)]
    pub to: Option<NaiveDate>,

    /// Filter by transaction type
    #[arg(long, value_enum)]
    pub tx_type: Option<TransactionType>,

    /// Filter by category
    #[arg(short, long)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, Display, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Pdf,
}

#[derive(Parser, Debug, Clone, Copy)]
pub struct RemoveArgs {
    /// filter by ID to remove the data from the database
    #[arg(long, value_enum)]
    pub id: i64,
}

impl DirhamlyCli {
    pub fn run(&self, db: &Database) -> Result<()> {
        match &self.command {
            Command::Add(args) => self.handle_add(db, args),
            Command::Read(args) => self.handle_read(db, args),
            Command::Export(args) => self.handle_export(db, args),
            Command::Remove(args) => self.remove_byid(db, args),
        }
    }

    fn handle_add(&self, db: &Database, args: &AddArgs) -> Result<()> {
        let date = args.date.unwrap_or_else(|| Local::now().date_naive());

        let transaction = Transaction::new(
            args.tx_type,
            args.amount,
            args.category.clone(),
            args.description.clone(),
            date,
        );

        db.add_transaction(&transaction)?;
        println!("Transaction added successfully.");

        Ok(())
    }

    fn handle_read(&self, db: &Database, args: &ReadArgs) -> Result<()> {
        let transactions =
            db.get_transactions(args.from, args.to, args.tx_type, args.category.clone())?;

        format_list(&transactions);
        Ok(())
    }

    fn handle_export(&self, db: &Database, args: &ExportArgs) -> Result<()> {
        let transactions =
            db.get_transactions(args.from, args.to, args.tx_type, args.category.clone())?;

        if transactions.is_empty() {
            println!("No transactions match the specified filters.");
            return Ok(());
        }

        export_transactions(&transactions, args.format)?;
        Ok(())
    }
    fn remove_byid(&self, db: &Database, args: &RemoveArgs) -> Result<()> {
        let rows = db.delete_by_id(args)?;
        println!("Deleted {} row(s) for id {}", rows, args.id);
        Ok(())
    }
}
