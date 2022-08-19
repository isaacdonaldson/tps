use anyhow::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Write};

// allow for copying, equality testing and sorting
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ClientId(u16);

// Enables printing
impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Client {
    pub id: ClientId,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl Client {
    pub fn new(id: ClientId) -> Self {
        Self {
            id,
            available: Decimal::new(0, 4), // 4 decimal places
            held: Decimal::new(0, 4),
            total: Decimal::new(0, 4),
            locked: false,
        }
    }

    pub fn new_with_values(
        id: ClientId,
        available: Decimal,
        held: Decimal,
        total: Decimal,
        locked: bool,
    ) -> Self {
        Self {
            id,
            available,
            held,
            total,
            locked,
        }
    }

    pub fn check_client_validity(&self) -> bool {
        let zero_val = Decimal::new(0, 4);

        let available_amount = self.total - self.held;
        if self.available < zero_val || available_amount != self.available {
            return false;
        }

        let held_amount = self.total - self.available;
        if self.held < zero_val || held_amount != self.held {
            return false;
        }

        let total_amount = self.available + self.held;
        if self.total < zero_val || total_amount != self.total {
            return false;
        }

        true
    }
}

// Holds a BTreeMap of ClientId to Client
// If this was in a concurrent/mutli-threaded environment, this would be an
// Arc<Mutex<BTreeMap<ClientId, Client>>>
#[derive(Debug)]
pub struct ClientPool {
    clients: BTreeMap<ClientId, Client>,
}

impl Default for ClientPool {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientPool {
    pub fn new() -> Self {
        Self {
            clients: BTreeMap::new(),
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.insert(client.id, client);
    }

    pub fn has_client(&self, client_id: &ClientId) -> Result<bool> {
        Ok(self.clients.contains_key(client_id))
    }

    pub fn get_client(&self, client_id: ClientId) -> Option<&Client> {
        self.clients.get(&client_id)
    }

    pub fn get_client_mut(&mut self, client_id: ClientId) -> Option<&mut Client> {
        self.clients.get_mut(&client_id)
    }

    pub fn format_for_print(&self) -> Result<String> {
        let mut output = String::from("client, available, held, total, locked\n");
        for (_, client) in self.clients.iter() {
            writeln!(
                &mut output,
                "{}, {1:.4}, {2:.4}, {3:.4}, {4}", // printing to 4 decimal places
                client.id, client.available, client.held, client.total, client.locked
            )?;
        }
        Ok(output)
    }
}
