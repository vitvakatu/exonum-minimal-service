extern crate env_logger;
#[macro_use]
extern crate exonum;

/// Useful imports.
use exonum::blockchain::{Service, Transaction, TransactionSet};
use exonum::crypto::Hash;
use exonum::encoding::Error;
use exonum::helpers::fabric::{self, Context, NodeBuilder};
use exonum::messages::RawMessage;
use exonum::storage::Snapshot;

mod schema;
mod transactions;

/// Our not-so-featureful service.
struct MinimalService;

impl Service for MinimalService {
    /// Unique service ID.
    fn service_id(&self) -> u16 {
        0
    }

    /// Unique service name.
    fn service_name(&self) -> &str {
        "minimal"
    }

    /// State hash of merkelized indexes.
    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    /// Function to convert transactions to our own type.
    fn tx_from_raw(&self, raw: RawMessage) -> Result<Box<Transaction>, Error> {
        transactions::MinimalTransactions::tx_from_raw(raw).map(Into::into)
    }
}

/// `ServiceFactory` to create our `MinimalService` on blockchain start.
struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(MinimalService)
    }
}

fn main() {
    env_logger::init();
    // Starting blockchain with our service.
    NodeBuilder::new()
        .with_service(Box::new(ServiceFactory))
        .run();
}
