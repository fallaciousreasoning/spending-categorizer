#[derive(RustcDecodable, Clone)]
pub struct StatementRow {
    pub account : String,
    pub date : String,
    pub description : String,
    pub source_code : String,
    pub this_reference : String,
    pub this_particulars : String,
    pub this_code : String,
    pub other_reference : String,
    pub other_particulars : String,
    pub other_code : String,
    pub other_name : String,
    pub other_account : String,
    pub credit_amount : String,
    pub debit_amount : String,
    pub total_amount : f32,
    pub balance : f32,
}