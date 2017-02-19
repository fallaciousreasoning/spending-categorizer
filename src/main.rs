extern crate csv;
extern crate rustc_serialize;

mod statement;
mod analyser;

fn main() {
    let mut rdr = csv::Reader::from_file("/Users/jayharris/money/data/statement.csv").unwrap();
    
    for record in rdr.decode() {
        let row : statement::StatementRow = record.unwrap();
        let description = row.description.to_string();
        let categorizers = analyser::initDefaultCategorizers();
        let result = analyser::categorize(row.description, categorizers);
        let len = result.len();

        for category in result {
            print!("{} -> {:?}, ", description, category);
        }
        
        if len > 0 { println!(); }
    }
}