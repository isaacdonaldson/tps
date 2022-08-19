use std::process;

use tps::{clients, read_buffer_to_csv, transactions};

fn main() {
    let args_vec: Vec<String> = std::env::args().collect();

    if args_vec.len() != 2 {
        eprintln!("incorrect usage of the interface, please provide options in the format 'cargo run -- <input_file.csv>'"
        );
        process::exit(1);
    }

    let input_csv_filename = &args_vec[1];

    // We can use the file buffer to read the CSV file into a vector of transactions.
    let csv_content = match read_buffer_to_csv(input_csv_filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "could not read csv contents from created file buffer due to: {}",
                e
            );
            process::exit(1);
        }
    };

    // create client pool to have transactions operate on
    // create transaction record
    // we want these to outlive the processing in case we need to store it
    let mut client_pool = clients::ClientPool::new();
    let mut transations = transactions::management::TransactionTree::new();

    //process the transactions
    transactions::processing::process_transactions(csv_content, &mut client_pool, &mut transations)
        .unwrap();

    // This prints out to stdout to allow the desired output behaviour
    match client_pool.format_for_print() {
        Ok(client_str) => (println!("{client_str}")),
        Err(e) => {
            eprintln!("could not print final client state due to: {}", e);
            process::exit(1);
        }
    };
}
