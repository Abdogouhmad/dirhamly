use crate::models::Transaction;

/// Utility function to format and display a list of transactions in a nice table format
/// This is used by the `list` command to show transactions in a user-friendly way.
/// It takes a slice of `Transaction` structs and prints them in a tabular format with headers.
pub fn format_list(transactions: &[Transaction]) {
    const ID_W: usize = 4;
    const DATE_W: usize = 12;
    const TYPE_W: usize = 12;
    const AMOUNT_W: usize = 15;
    const CATEGORY_W: usize = 12;
    if transactions.is_empty() {
        println!("No transactions found matching your criteria.");
        return;
    }

    // Print the Table Header
    println!(
        "{:<ID_W$} {:<DATE_W$} {:<TYPE_W$} {:<AMOUNT_W$} {:<CATEGORY_W$} {}",
        "ID", "Date", "Type", "Amount", "Category", "Description",
    );

    // Print the Divider Line
    println!("{}", "─".repeat(70));

    // Loop through the list and print each row
    for tx in transactions {
        let id_str = tx.id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string());
        let amount_str = format!("{:.2} MAD", tx.amount);

        println!(
            "{:<ID_W$} {:<DATE_W$} {:<TYPE_W$} {:<AMOUNT_W$} {:<CATEGORY_W$}{}",
            id_str,
            tx.date,
            tx.tx_type.to_string(),
            amount_str,
            tx.category.to_string(),
            tx.description
        );
    }
}
