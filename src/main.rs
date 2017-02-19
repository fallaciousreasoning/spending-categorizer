extern crate csv;
extern crate rustc_serialize;

mod statement;
mod analyser;

fn main() {
    let mut rdr = csv::Reader::from_file("C:/Users/Jay Harris/OneDrive/Documents/Projects/spending-categorizer/data/statement.csv").unwrap();
    let statement_rows : Vec<statement::StatementRow> = rdr.decode().map(|record| record.unwrap()).collect();
    let categorizers = analyser::from_json_file("C:/Users/Jay Harris/OneDrive/Documents/Projects/spending-categorizer/src/categorizers.json");
    let analyser = analyser::Analysis::new(&statement_rows, categorizers);

    for category in analyser.all_categories() {
        println!("{:?}", category);
        println!("\tSpent {}", analyser.total_spent(category));
    }
}