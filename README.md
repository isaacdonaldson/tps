# Transaction Processing System

This is a simple transaction processing system that processing 5 transaction types: Deposits, Withdrawals, Disputes, Resolutions, and Chargebacks. 

## Installation
This project uses version `1.63.0` of rust. To install the project, run the following command:

```bash
git clone https://github.com/isaacdonaldson/tps.git
cd tps
cargo install --path .
cargo run -- transactions.csv > output.csv
```

## Testing 
To test the project, run the following command:

```bash
cargo test
```


## Error Handling 
If the program encounters an error where the continuation of the program is impossible (command line argument is missing, provided file cannot be found, etc..) then it exits the process with a non-zero exit code. Errors where the program can continue are handled by logging the error to stderr using the `eprint!` macro. This allows the end result (output file) to contain only the desired info, but the errors are still printed out to the console. I make heavy use of the anyhow crate to handle errors as it is much easier to use than the standard library.



## Decimal Type
I use the rust-decimal crate because using floats for money is famously a bad idea. I use the decimal crate to prevent the precision errors floats have, and can easily use the proper precision of 4 decimal places without having to use much extra overhead.



## Design Considerations
The following are some design considerations I thought are worth mentioning.

### `BTreeMap`
Because we know that client ID's and transaction ID's are ordered numbers, we can use this fact to speed up the operations, and print ordered results, both are benefits when compared with a `HashMap`. This structure is also suitable when wrapped in other types like a `Mutex` or `RwLock`.

### Using the Type System
I used the type system quite a bit to provide clarity of intentions, but as well to limit the program to the defined behavior. For example, using an `Enum` for all possible transaction types allows the compiler to ensure that every transaction type is handled when using the `match` expression. In addition, creating types like `ClientPool` and `TransactionTree` allows me to control what functionality is available to the programmer, restricting behaviours I don't want or adding functionality I do. This allows for better maintainability, but also allows for more correctness in the program.

### Validity Checks and Faux Atomicity
I added some validity checks to the program to ensure that the program is behaving as expected. Beyond the basic ones like checking a client or transaction exists, after each transaction I check if the client's balances are valid as explained in the document. Becuase the transactions are mutable, if the transaction is invalid, the transaction will be 'reversed' and the balances will be returned to their pre-transaction amounts. This is the most basic implementation of Atomicity.

### Serde Serialization and Deserialization
Using `serde` allows me to avoid some error prone areas with data ingestion and outputting. The serialization capability allows me to define the data type, and allow serde to handle the edge cases, where errors can easily occur. This allows me to focus on designing proper types, and a more correct system.



## Possible Extensions
### Scale and Concurreny
Keeping scale and concurrency in mind, I made some decisions that should allow for easy changing if the need arises. I did not implement them all as I felt they were not necessary, and would have added uneeded complexity without much benefit due to not having clarity on what scale would mean in this case. The first extension was to read the csv using the `Reader` type `BufRead`. This reads the file into an internal buffer, allowing for better processing on large files as the whole file is no longer read into memory. This is generally a good extension, and quite easy to implement. Becuase this was implemented, the work needed to change the file buffer into a TCP buffer is very minimal, and can easily be added to enable the program to read from TCP sockets. If TCP sockets were being used, then there is a high likelhood that the program would be asynchronous, and the program would need to be able to handle multiple clients at the same time. This could be done by wrapping both the `TransactionTree` and `ClientPool` in a `Arc<Mutex<T>>`. That way, the updates to these trees would be safe from data races and could also be atomic across threads.


## Assumptions
### Failed Transactions continue program
If a transaction fails (ex: client has insufficient funds for a withdrawal), then the program should continue to process the rest of the transactions, only the failed transaction is skipped.

### Dispute, Resolve, and Chargebacks only occur on Deposit transactions
I assumed that disputes, resolutions, and chargebacks only occur on deposits. This is a reasonable assumption, as there is no clear way to handle these on other transaction types.

### Frozen Account Prevents Activity
I assumed that a frozen account prevents anymore transactions from being processed on it. So all 5 transaction types would be ignored for that account.

### Accounts can be created
I assumed that on failed transactions, the account can still be created and it have no effect on the output as long as the transaction did not effect the account at all (i.e. Account exists but balances are all 0).