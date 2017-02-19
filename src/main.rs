extern crate csv;
extern crate rustc_serialize;

mod statement;
mod analyser;

fn main() {
    let mut rdr = csv::Reader::from_file("C:/Users/Jay Harris/OneDrive/Documents/Projects/spending-categorizer/data/statement.csv").unwrap();
    let statementRows : Vec<statement::StatementRow> = rdr.decode().map(|record| record.unwrap()).collect();
    let categorizers = analyser::defaultCategorizers();
    let analyser = analyser::Analysis::new(&statementRows, categorizers);
    
    for statement in analyser.get_statements(analyser::Category::Supermarket).unwrap() {
        println!("Spent {} at {}", -statement.total_amount, statement.description);
    }
}