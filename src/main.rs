mod commands;
mod db;
mod models;
mod utils;

use clap::Parser;
use commands::DirhamlyCli;
use db::Database;
fn main() {
    let db = Database::new("dirhamly.db").expect("Failed to connect to database");
    db.initialize().expect("Failed to initialize database");

    let cli = DirhamlyCli::parse();
    cli.run(&db);
}
