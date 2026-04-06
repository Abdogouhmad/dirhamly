use crate::commands::ExportFormat;
use crate::models::Transaction;
use crate::report::{Report, generate_report};

use anyhow::{Context, Result};
use num_traits::ToPrimitive;

use std::fs::File;
use std::io::Write;
use typst_bake::{IntoDict, IntoValue, document};

/// Utility function to format and display a list of transactions in a nice table.
pub fn format_list(transactions: &[Transaction]) {
    if transactions.is_empty() {
        println!("No transactions found.");
        return;
    }

    const ID_W: usize = 6;
    const DATE_W: usize = 12;
    const TYPE_W: usize = 10;
    const AMOUNT_W: usize = 14;
    const CAT_W: usize = 16;
    const DESC_W: usize = 40;

    let total_width = ID_W + DATE_W + TYPE_W + AMOUNT_W + CAT_W + DESC_W + 11; // padding/separators

    println!(
        "{:<ID_W$}  {:<DATE_W$}  {:<TYPE_W$}  {:>AMOUNT_W$}  {:<CAT_W$}  {:<DESC_W$}",
        "ID", "Date", "Type", "Amount (MAD)", "Category", "Description"
    );
    println!("{}", "─".repeat(total_width));

    for tx in transactions {
        let id_str = tx.id.map_or("-".to_string(), |id| id.to_string());
        let amount_str = format!("{:+.2}", tx.amount); // shows + for income

        println!(
            "{:<ID_W$}  {:<DATE_W$}  {:<TYPE_W$}  {:>AMOUNT_W$}  {:<CAT_W$}  {:<DESC_W$}",
            id_str,
            tx.date,
            tx.tx_type.to_string(),
            amount_str,
            tx.category,
            tx.description.chars().take(DESC_W).collect::<String>() // truncate long desc
        );
    }
}

/// Export transactions to CSV or PDF using the report structure.
pub fn export_transactions(
    transactions: &[Transaction],
    format: ExportFormat,
    // Tip: later add → output_path: Option<&str>
) -> Result<()> {
    let report = generate_report(transactions);

    match format {
        ExportFormat::Csv => export_to_csv(&report)?,
        ExportFormat::Pdf => export_to_pdf(&report)?,
    }

    Ok(())
}

fn export_to_csv(report: &Report) -> Result<()> {
    let mut file = File::create("report.csv").context("Failed to create report.csv")?;

    // Better CSV header + escaping
    writeln!(file, "ID,Type,Category,Amount_MAD,Date,Description")?;

    for tx in &report.transactions {
        // Simple escaping: wrap fields that may contain commas/quotes
        let desc = tx.description.replace('"', "\"\""); // basic CSV escape

        writeln!(
            file,
            "{},\"{}\",\"{}\",{:.2},\"{}\",\"{}\"",
            tx.id,
            tx.tx_type,
            tx.category.replace('"', "\"\""),
            tx.amount,
            tx.date,
            desc
        )?;
    }

    println!("CSV exported → report.csv");
    println!("  Total income   : {:+.2} MAD", report.total_income);
    println!("  Total expenses : {:.2} MAD", report.total_expenses);
    println!("  Balance        : {:+.2} MAD", report.balance);
    println!("  Transactions   : {}", report.transaction_count);

    Ok(())
}

// Define the struct for Typst inputs matching the macro requirements
#[derive(IntoValue, IntoDict)]
struct ReportInputs {
    data: String,
}

fn export_to_pdf(report: &Report) -> Result<()> {
    // 1. Assign the JSON macro to a named variable `data` instead of `_`
    let data = serde_json::json!({
        "total_income": report.total_income.to_f64().unwrap_or(0.0),
        "total_expenses": report.total_expenses.to_f64().unwrap_or(0.0),
        "balance": report.balance.to_f64().unwrap_or(0.0),
        "transaction_count": report.transaction_count,

        "transactions": report.transactions.iter().map(|t| {
            serde_json::json!({
                "id": t.id,
                "tx_type": t.tx_type,
                "category": t.category,
                "description": t.description,
                "amount": t.amount.to_f64().unwrap_or(0.0),
                "date": t.date,
            })
        }).collect::<Vec<_>>(),

        "category_breakdown": report.category_breakdown.iter().map(|c| {
            serde_json::json!({
                "category": c.category,
                "amount": c.amount.to_f64().unwrap_or(0.0),
                "percentage": c.percentage.round(),
            })
        }).collect::<Vec<_>>(),
    });

    // 2. Convert it to a String
    let json_str = data.to_string();

    // 3. Inject JSON data into Typst's system inputs using the specific struct
    let inputs = ReportInputs { data: json_str };

    // Use typst-bake's document! macro (compile-time baked template) and attach the inputs
    let doc = document!("report.typ").with_inputs(inputs);

    // 4. Generate the PDF
    let pdf_bytes = doc.to_pdf().context("Failed to compile Typst document to PDF")?;

    let output_path = "report.pdf";
    std::fs::write(output_path, pdf_bytes).context("Failed to write PDF file")?;

    println!("PDF report generated → {}", output_path);

    Ok(())
}
