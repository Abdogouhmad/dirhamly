use crate::models::Transaction;

/// Utility function to format and display a list of transactions
/// This function calculates column widths and prints a well-formatted table of transactions.
/// It handles empty lists gracefully and ensures that the output is aligned and easy to read.
pub fn format_list(transactions: &[Transaction]) {
    const ID_W: usize = 4;
    const DATE_W: usize = 14;
    const TYPE_W: usize = 12;
    const AMOUNT_W: usize = 15;
    const CATEGORY_W: usize = 14;
    const DESC_W: usize = 20; // ← give it real width

    if transactions.is_empty() {
        println!("No transactions found matching your criteria.");
        return;
    }

    // Calculate full table width dynamically
    let total_width =
        ID_W + DATE_W + TYPE_W + AMOUNT_W + CATEGORY_W + DESC_W + 5;

    // Header
    println!(
        "{:<ID_W$} {:<DATE_W$} {:<TYPE_W$} {:<AMOUNT_W$} {:<CATEGORY_W$} {:<DESC_W$}",
        "ID", "Date", "Type", "Amount", "Category", "Description",
    );

    // Divider
    println!("{}", "─".repeat(total_width));

    // Rows
    for tx in transactions {
        let id_str = tx.id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string());
        let amount_str = format!("{:.2} MAD", tx.amount);

        println!(
            "{:<ID_W$} {:<DATE_W$} {:<TYPE_W$} {:>AMOUNT_W$} {:<CATEGORY_W$} {:<DESC_W$}",
            id_str,
            tx.date,
            tx.tx_type.to_string(),
            amount_str,          // right aligned (better for money)
            tx.category.to_string(),
            tx.description
        );
    }
}