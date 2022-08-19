use crate::clients::{Client, ClientPool};
use anyhow::Result;
use rust_decimal::prelude::*;

use super::{management::TransactionTree, Transaction, TransactionType};

pub fn process_transactions(
    transactions: Vec<Transaction>,
    clients: &mut ClientPool,
    transaction_numbers: &mut TransactionTree,
) -> Result<()> {
    for transaction in transactions {
        // Since dispute types don't have a transaction id
        // we only check for deposits and withdrawals
        if (transaction.tx_type == TransactionType::Deposit
            || transaction.tx_type == TransactionType::Withdrawal)
            && transaction_numbers.contains(&transaction.tx_id)
        {
            eprintln!("Duplicate transaction: {:?}", &transaction.tx_id);
            continue;
        }

        match &transaction.tx_type {
            TransactionType::Deposit => match process_deposit(transaction, clients) {
                // Only add the transaction to the tree if it was successfully processed
                Ok(_) => transaction_numbers.insert(transaction),
                Err(e) => {
                    // Making the decision here to continue processing on an error.
                    eprintln!(
                        "error processing deposit {:?}, skipping due to '{}'",
                        &transaction.tx_id, e
                    );
                }
            },
            TransactionType::Withdrawal => match process_withdrawal(transaction, clients) {
                Ok(_) => transaction_numbers.insert(transaction),
                Err(e) => {
                    eprintln!(
                        "error processing withdrawal {:?}, skipping due to '{}'",
                        &transaction.tx_id, e
                    );
                }
            },
            TransactionType::Dispute => {
                match process_dispute(transaction, clients, transaction_numbers) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "error processing dispute for transaction {:?}, skipping due to '{}'",
                            &transaction.tx_id, e
                        );
                    }
                }
            }
            TransactionType::Resolve => {
                match process_resolve(transaction, clients, transaction_numbers) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "error processing resolve for transaction {:?}, skipping due to '{}'",
                            &transaction.tx_id, e
                        );
                    }
                }
            }
            TransactionType::Chargeback => {
                match process_chargeback(transaction, clients, transaction_numbers) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "error processing chargeback for transaction {:?}, skipping due to '{}'",
                            &transaction.tx_id, e
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn process_deposit(transaction: Transaction, clients: &mut ClientPool) -> Result<()> {
    let found_client = clients.has_client(&transaction.client_id)?;

    if !found_client {
        // add client to pool
        let new_client = Client::new(transaction.client_id);
        clients.add_client(new_client);
    }

    // this is now guarenteed to not be None
    let client = clients
        .get_client_mut(transaction.client_id)
        .ok_or_else(|| anyhow::anyhow!("client not found in pool: {:?}", transaction.client_id))?;

    // locked accounts should not continue
    if client.locked {
        return Err(anyhow::anyhow!(
            "client {:?} is locked, cannot process deposit",
            transaction.client_id
        ));
    }

    // deposits should always have an amount
    let deposit_amount = transaction
        .amount
        .ok_or_else(|| anyhow::anyhow!("Error: Transaction amount was not provided",))?;

    if deposit_amount < Decimal::from(0) {
        return Err(anyhow::anyhow!("Error: Deposit amount is negative"));
    }

    // deposit amount to client available balance
    client.available += deposit_amount;
    client.total += deposit_amount;

    if !client.check_client_validity() {
        // return amounts back
        client.available -= deposit_amount;
        client.total -= deposit_amount;
        return Err(anyhow::anyhow!("Error: Client is invalid after deposit",));
    }

    Ok(())
}

fn process_withdrawal(transaction: Transaction, clients: &mut ClientPool) -> Result<()> {
    let found_client = clients.has_client(&transaction.client_id)?;

    if !found_client {
        // add client to pool, this is okay for withdrawals without an
        // existing client because it will create the client but it
        // will not process the transaction
        let new_client = Client::new(transaction.client_id);
        clients.add_client(new_client);
    }

    // this is now guarenteed to not be None
    let client = clients
        .get_client_mut(transaction.client_id)
        .ok_or_else(|| anyhow::anyhow!("client not found in pool: {:?}", transaction.client_id))?;

    // locked accounts should not continue
    if client.locked {
        return Err(anyhow::anyhow!(
            "client {:?} is locked, cannot process withdrawal",
            transaction.client_id
        ));
    }

    let withdrawal_amount = transaction
        .amount
        .ok_or_else(|| anyhow::anyhow!("Error: Transaction amount was not provided",))?;

    if withdrawal_amount < Decimal::from(0) {
        return Err(anyhow::anyhow!("Error: withdrawal amount is negative"));
    }

    // Check to see if the client has enough available balance to withdraw
    if client.available < withdrawal_amount {
        return Err(anyhow::anyhow!(
            "Error: Client does not have enough available balance to withdraw",
        ));
    }

    // withdrawal amount to client available balance
    // This mutates so we need to be sure that the transaction
    // is valid before and after we do this
    client.available -= withdrawal_amount;
    client.total -= withdrawal_amount;

    if !client.check_client_validity() {
        // if the client is invalid after the withdrawal, we need to put it back
        client.available += withdrawal_amount;
        client.total += withdrawal_amount;
        return Err(anyhow::anyhow!("Error: Client is invalid after withdrawal",));
    }

    Ok(())
}

fn process_dispute(
    transaction: Transaction,
    clients: &mut ClientPool,
    transaction_tree: &mut TransactionTree,
) -> Result<()> {
    let found_client = clients.has_client(&transaction.client_id)?;

    if !found_client {
        // we don't want to create a client in this case
        // we will just return an error and ignore the transaction
        return Err(anyhow::anyhow!(
            "Error: Client not found in pool: {:?}",
            transaction.client_id
        ));
    }

    // Need to get the actual transaction to get the details
    // the dispute transaction only has the transaction id
    let found_transaction = match transaction_tree.get_mut(&transaction.tx_id) {
        Some(tx) => tx,
        None => {
            return Err(anyhow::anyhow!(
                "Error: Provided transaction has not been processed"
            ));
        }
    };

    // like others, we know this will succeed
    let client = clients
        .get_client_mut(transaction.client_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Error: Client not found in pool: {:?}",
                transaction.client_id
            )
        })?;

    // locked accounts should not continue
    if client.locked {
        return Err(anyhow::anyhow!(
            "client {:?} is locked, cannot process dispute",
            transaction.client_id
        ));
    }

    let dispute_amount = found_transaction
        .amount
        .ok_or_else(|| anyhow::anyhow!("Error: Transaction amount was not provided",))?;

    // It only makes sense to dispute a deposit
    if found_transaction.tx_type == TransactionType::Deposit {
        // check to see if the client has enough available balance to dispute
        if client.available < dispute_amount {
            return Err(anyhow::anyhow!(
                "Error: Client does not have enough available balance to dispute",
            ));
        }
        // dispute amount to client available balance
        client.available -= dispute_amount;
        client.held += dispute_amount;
        // change to show the transaction is now disputed
        found_transaction.in_dispute = true;

        if !client.check_client_validity() {
            // if the client is invalid after the dispute, we need to put it back
            client.available += dispute_amount;
            client.held -= dispute_amount;
            found_transaction.in_dispute = false;

            return Err(anyhow::anyhow!("Error: Client is invalid after dispute",));
        }
    }

    Ok(())
}

fn process_resolve(
    transaction: Transaction,
    clients: &mut ClientPool,
    transaction_tree: &mut TransactionTree,
) -> Result<()> {
    let found_client = clients.has_client(&transaction.client_id)?;

    if !found_client {
        return Err(anyhow::anyhow!(
            "Error: Client not found in pool: {:?}",
            transaction.client_id
        ));
    }

    // Need to get the actual transaction to get the details
    // the resolve transaction only has the transaction id
    let found_transaction = match transaction_tree.get_mut(&transaction.tx_id) {
        Some(tx) => tx,
        None => {
            return Err(anyhow::anyhow!(
                "Error: Provided transaction has not been processed"
            ));
        }
    };

    // like others, we know this will succeed
    let client = clients
        .get_client_mut(transaction.client_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Error: Client not found in pool: {:?}",
                transaction.client_id
            )
        })?;

    // locked accounts should not continue
    if client.locked {
        return Err(anyhow::anyhow!(
            "client {:?} is locked, cannot process resolve",
            transaction.client_id
        ));
    }

    let resolve_amount = found_transaction
        .amount
        .ok_or_else(|| anyhow::anyhow!("Error: Transaction amount was not provided",))?;

    // Since only deposits can be disputed, it only makes sense to resolve a deposit
    if found_transaction.tx_type == TransactionType::Deposit {
        // check to see if the client has enough held funds to process the dispute
        if client.held < resolve_amount {
            return Err(anyhow::anyhow!(
                "Error: Client does not have enough held funds to resolve",
            ));
        }

        if !found_transaction.in_dispute {
            return Err(anyhow::anyhow!(
                "Error: Specified transaction is not in dispute",
            ));
        }

        // return the disputed amount to available balance from held balance
        client.available += resolve_amount;
        client.held -= resolve_amount;
        // change to show the transaction is no longer disputed
        found_transaction.in_dispute = false;

        if !client.check_client_validity() {
            // if the client is invalid after the resolve, we need to put it back
            client.available -= resolve_amount;
            client.held += resolve_amount;
            found_transaction.in_dispute = true;

            return Err(anyhow::anyhow!("Error: Client is invalid after resolve",));
        }
    }

    Ok(())
}

fn process_chargeback(
    transaction: Transaction,
    clients: &mut ClientPool,
    transaction_tree: &mut TransactionTree,
) -> Result<()> {
    let found_client = clients.has_client(&transaction.client_id)?;

    if !found_client {
        return Err(anyhow::anyhow!(
            "Error: Client not found in pool: {:?}",
            transaction.client_id
        ));
    }

    let found_transaction = match transaction_tree.get_mut(&transaction.tx_id) {
        Some(tx) => tx,
        None => {
            return Err(anyhow::anyhow!(
                "Error: Provided transaction has not been processed"
            ));
        }
    };

    // like others, we know this will succeed
    let client = clients
        .get_client_mut(transaction.client_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Error: Client not found in pool: {:?}",
                transaction.client_id
            )
        })?;

    // locked accounts should not continue
    if client.locked {
        return Err(anyhow::anyhow!(
            "client {:?} is locked, cannot process chargeback",
            transaction.client_id
        ));
    }

    let chargeback_amount = found_transaction
        .amount
        .ok_or_else(|| anyhow::anyhow!("Error: Transaction amount was not provided",))?;

    // Since only deposits can be disputed, it only makes sense to chargeback a deposit
    if found_transaction.tx_type == TransactionType::Deposit {
        // check to see if the client has enough held funds to process the chargeback
        if client.held < chargeback_amount {
            return Err(anyhow::anyhow!(
                "Error: Client does not have enough held funds to chargeback",
            ));
        }

        if !found_transaction.in_dispute {
            return Err(anyhow::anyhow!(
                "Error: Specified transaction is not in dispute",
            ));
        }

        // this is teh chargeback, client gets the money back and we subtract
        client.held -= chargeback_amount;
        client.total -= chargeback_amount;
        // Chargebacks do freeze the account though
        client.locked = true;

        // change to show the transaction is no longer disputed
        found_transaction.in_dispute = false;

        if !client.check_client_validity() {
            // if the client is invalid after the resolve, we need to put it back
            client.held += chargeback_amount;
            client.total += chargeback_amount;
            found_transaction.in_dispute = true;

            client.locked = false;

            return Err(anyhow::anyhow!("Error: Client is invalid after chargeback",));
        }
    }

    Ok(())
}
