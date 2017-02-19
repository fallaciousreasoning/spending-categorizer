extern crate csv;
extern crate rustc_serialize;

mod statement;
mod analyser;

fn main() {
    let mut rdr = csv::Reader::from_file("/Users/jayharris/money/data/statement.csv").unwrap();
    for record in rdr.decode() {
        let row : statement::StatementRow = record.unwrap();
        println!("{} for {} leaving {}", row.description, row.total_amount, row.balance);
    }
}