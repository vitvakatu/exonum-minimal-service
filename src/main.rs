// Copyright 2018 Ilya Bogdanov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate env_logger;
#[macro_use]
extern crate exonum;
extern crate bodyparser;
extern crate iron;
extern crate router;
extern crate serde_json;

/// Useful imports.
use exonum::api::Api;
use exonum::blockchain::{ApiContext, Service, Transaction, TransactionSet};
use exonum::crypto::Hash;
use exonum::encoding::Error;
use exonum::helpers::fabric::{self, Context, NodeBuilder};
use exonum::messages::RawMessage;
use exonum::storage::Snapshot;

use iron::Handler;
use router::Router;

mod api;
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

    fn public_api_handler(&self, context: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let blockchain = context.blockchain().clone();
        let channel = context.node_channel().clone();
        let api = api::MinimalApi::new(blockchain, channel);
        api.wire(&mut router);
        Some(Box::new(router))
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
