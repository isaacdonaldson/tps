use tps::{clients, read_buffer_to_csv, transactions};

#[cfg(test)]
#[test]
#[should_panic]
fn read_err() {
    let input_csv_filename = "null_file_name.csv";

    read_buffer_to_csv(input_csv_filename).unwrap();
}

#[cfg(test)]
#[test]
fn basic_deposit_ignore() {
    let input_csv_filename = "tests/t1_transactions.csv";

    let csv_content = read_buffer_to_csv(input_csv_filename).unwrap();

    let mut client_pool = clients::ClientPool::new();
    let mut transations = transactions::management::TransactionTree::new();

    //process the transactions
    transactions::processing::process_transactions(csv_content, &mut client_pool, &mut transations)
        .unwrap();

    // check here the values in the client pool
    let expected_result = r#"client, available, held, total, locked
1, 1.0000, 0.0000, 1.0000, false
2, 0.0000, 0.0000, 0.0000, false
"#;

    assert_eq!(client_pool.format_for_print().unwrap(), expected_result);
}

#[cfg(test)]
#[test]
fn my_test_data() {
    let input_csv_filename = "tests/t0_transactions.csv";

    let csv_content = read_buffer_to_csv(input_csv_filename).unwrap();

    let mut client_pool = clients::ClientPool::new();
    let mut transations = transactions::management::TransactionTree::new();

    //process the transactions
    transactions::processing::process_transactions(csv_content, &mut client_pool, &mut transations)
        .unwrap();

    // check here the values in the client pool
    let expected_result = r#"client, available, held, total, locked
1, 2.5000, 0.0000, 2.5000, false
2, 0.0234, 0.0000, 0.0234, true
3, 0.0000, 0.0000, 0.0000, false
"#;

    assert_eq!(client_pool.format_for_print().unwrap(), expected_result);
}

#[cfg(test)]
#[test]
fn basic_dispute() {
    let input_csv_filename = "tests/t2_transactions.csv";

    let csv_content = read_buffer_to_csv(input_csv_filename).unwrap();

    let mut client_pool = clients::ClientPool::new();
    let mut transations = transactions::management::TransactionTree::new();

    //process the transactions
    transactions::processing::process_transactions(csv_content, &mut client_pool, &mut transations)
        .unwrap();

    // check here the values in the client pool
    let expected_result = r#"client, available, held, total, locked
1, 0.5000, 1.0000, 1.5000, false
2, 2.0000, 0.0000, 2.0000, false
"#;

    assert_eq!(client_pool.format_for_print().unwrap(), expected_result);
}

#[cfg(test)]
#[test]
fn advanced_dispute() {
    let input_csv_filename = "tests/t3_transactions.csv";

    let csv_content = read_buffer_to_csv(input_csv_filename).unwrap();

    let mut client_pool = clients::ClientPool::new();
    let mut transations = transactions::management::TransactionTree::new();

    //process the transactions
    transactions::processing::process_transactions(csv_content, &mut client_pool, &mut transations)
        .unwrap();

    // Cases covered here:
    // - duplicate transaction ids
    // - dispute on withdrawals (ignored)
    // - withdrawal without enough $
    // - locked client
    // - not enough funds to dispute

    // check here the values in the client pool
    let expected_result = r#"client, available, held, total, locked
1, 0.3000, 0.0000, 0.3000, true
2, 1.1250, 0.0000, 1.1250, false
"#;

    assert_eq!(client_pool.format_for_print().unwrap(), expected_result);
}

#[cfg(test)]
#[test]
fn many_transactions() {
    let input_csv_filename = "tests/t4_transactions.csv";

    let csv_content = read_buffer_to_csv(input_csv_filename).unwrap();

    let mut client_pool = clients::ClientPool::new();
    let mut transations = transactions::management::TransactionTree::new();

    //process the transactions
    transactions::processing::process_transactions(csv_content, &mut client_pool, &mut transations)
        .unwrap();

    // check here the values in the client pool
    let expected_result = r#"client, available, held, total, locked
1, 1.0000, 0.0000, 1.0000, false
"#;

    assert_eq!(client_pool.format_for_print().unwrap(), expected_result);
}

#[cfg(test)]
#[test]
fn many_transactions_many_accounts() {
    let input_csv_filename = "tests/t5_transactions.csv";

    let csv_content = read_buffer_to_csv(input_csv_filename).unwrap();

    let mut client_pool = clients::ClientPool::new();
    let mut transations = transactions::management::TransactionTree::new();

    //process the transactions
    transactions::processing::process_transactions(csv_content, &mut client_pool, &mut transations)
        .unwrap();

    // check here the values in the client pool
    let expected_result = r#"client, available, held, total, locked
1, 1.0000, 0.0000, 1.0000, false
2, 1.0000, 0.0000, 1.0000, false
3, 1.0000, 0.0000, 1.0000, false
4, 1.0000, 0.0000, 1.0000, false
5, 1.0000, 0.0000, 1.0000, false
6, 1.0000, 0.0000, 1.0000, false
7, 1.0000, 0.0000, 1.0000, false
8, 1.0000, 0.0000, 1.0000, false
9, 1.0000, 0.0000, 1.0000, false
10, 1.0000, 0.0000, 1.0000, false
11, 1.0000, 0.0000, 1.0000, false
12, 1.0000, 0.0000, 1.0000, false
13, 1.0000, 0.0000, 1.0000, false
14, 1.0000, 0.0000, 1.0000, false
15, 1.0000, 0.0000, 1.0000, false
16, 1.0000, 0.0000, 1.0000, false
17, 1.0000, 0.0000, 1.0000, false
18, 1.0000, 0.0000, 1.0000, false
19, 1.0000, 0.0000, 1.0000, false
20, 1.0000, 0.0000, 1.0000, false
21, 1.0000, 0.0000, 1.0000, false
22, 1.0000, 0.0000, 1.0000, false
23, 1.0000, 0.0000, 1.0000, false
24, 1.0000, 0.0000, 1.0000, false
25, 1.0000, 0.0000, 1.0000, false
26, 1.0000, 0.0000, 1.0000, false
27, 1.0000, 0.0000, 1.0000, false
28, 1.0000, 0.0000, 1.0000, false
29, 1.0000, 0.0000, 1.0000, false
30, 1.0000, 0.0000, 1.0000, false
31, 1.0000, 0.0000, 1.0000, false
32, 1.0000, 0.0000, 1.0000, false
33, 1.0000, 0.0000, 1.0000, false
34, 1.0000, 0.0000, 1.0000, false
35, 1.0000, 0.0000, 1.0000, false
36, 1.0000, 0.0000, 1.0000, false
37, 1.0000, 0.0000, 1.0000, false
38, 1.0000, 0.0000, 1.0000, false
39, 1.0000, 0.0000, 1.0000, false
40, 1.0000, 0.0000, 1.0000, false
41, 1.0000, 0.0000, 1.0000, false
42, 1.0000, 0.0000, 1.0000, false
43, 1.0000, 0.0000, 1.0000, false
44, 1.0000, 0.0000, 1.0000, false
45, 1.0000, 0.0000, 1.0000, false
46, 1.0000, 0.0000, 1.0000, false
47, 1.0000, 0.0000, 1.0000, false
48, 1.0000, 0.0000, 1.0000, false
49, 1.0000, 0.0000, 1.0000, false
50, 1.0000, 0.0000, 1.0000, false
51, 1.0000, 0.0000, 1.0000, false
52, 1.0000, 0.0000, 1.0000, false
53, 1.0000, 0.0000, 1.0000, false
54, 1.0000, 0.0000, 1.0000, false
55, 1.0000, 0.0000, 1.0000, false
56, 1.0000, 0.0000, 1.0000, false
57, 1.0000, 0.0000, 1.0000, false
58, 1.0000, 0.0000, 1.0000, false
59, 1.0000, 0.0000, 1.0000, false
60, 1.0000, 0.0000, 1.0000, false
61, 1.0000, 0.0000, 1.0000, false
62, 1.0000, 0.0000, 1.0000, false
63, 1.0000, 0.0000, 1.0000, false
64, 1.0000, 0.0000, 1.0000, false
65, 1.0000, 0.0000, 1.0000, false
66, 1.0000, 0.0000, 1.0000, false
67, 1.0000, 0.0000, 1.0000, false
68, 1.0000, 0.0000, 1.0000, false
69, 1.0000, 0.0000, 1.0000, false
70, 1.0000, 0.0000, 1.0000, false
71, 1.0000, 0.0000, 1.0000, false
72, 1.0000, 0.0000, 1.0000, false
73, 1.0000, 0.0000, 1.0000, false
74, 1.0000, 0.0000, 1.0000, false
75, 1.0000, 0.0000, 1.0000, false
76, 1.0000, 0.0000, 1.0000, false
77, 1.0000, 0.0000, 1.0000, false
78, 1.0000, 0.0000, 1.0000, false
79, 1.0000, 0.0000, 1.0000, false
80, 1.0000, 0.0000, 1.0000, false
81, 1.0000, 0.0000, 1.0000, false
82, 1.0000, 0.0000, 1.0000, false
83, 1.0000, 0.0000, 1.0000, false
84, 1.0000, 0.0000, 1.0000, false
85, 1.0000, 0.0000, 1.0000, false
86, 1.0000, 0.0000, 1.0000, false
87, 1.0000, 0.0000, 1.0000, false
88, 1.0000, 0.0000, 1.0000, false
89, 1.0000, 0.0000, 1.0000, false
90, 1.0000, 0.0000, 1.0000, false
91, 1.0000, 0.0000, 1.0000, false
92, 1.0000, 0.0000, 1.0000, false
93, 1.0000, 0.0000, 1.0000, false
94, 1.0000, 0.0000, 1.0000, false
95, 1.0000, 0.0000, 1.0000, false
96, 1.0000, 0.0000, 1.0000, false
97, 1.0000, 0.0000, 1.0000, false
98, 1.0000, 0.0000, 1.0000, false
99, 1.0000, 0.0000, 1.0000, false
"#;

    assert_eq!(client_pool.format_for_print().unwrap(), expected_result);
}
