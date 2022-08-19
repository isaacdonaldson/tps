use anyhow::Result;
use std::io;

pub mod clients;
pub mod transactions;

pub fn read_buffer_to_csv(filename: &str) -> Result<Vec<transactions::Transaction>> {
    // Creating a BufReader to read the file will help
    // on memory usage and performance for large files.
    let file = std::fs::File::open(filename)?;
    let buf = io::BufReader::new(file);

    // Construct a CSV reader from the BufReader that trims the whitespace for each field.
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(buf);

    let mut trans = Vec::new();

    for result in reader.deserialize() {
        let record: transactions::Transaction = result?;
        trans.push(record);
    }

    Ok(trans)
}
