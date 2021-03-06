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

use exonum::api::{Api, ApiError};
use exonum::blockchain::{Blockchain, Transaction};
use exonum::node::TransactionSend;

use bodyparser;
use iron::prelude::*;
use router::Router;
use serde_json;

use schema::DatabaseSchema;
use transactions::MinimalTransactions;

#[derive(Clone)]
pub struct MinimalApi<T: TransactionSend + Clone> {
    /// Exonum blockchain.
    pub blockchain: Blockchain,
    /// Channel for transactions.
    pub channel: T,
}

impl<T> MinimalApi<T>
where
    T: TransactionSend + Clone,
{
    pub fn new(blockchain: Blockchain, channel: T) -> Self {
        Self {
            blockchain,
            channel,
        }
    }
}

impl<T> Api for MinimalApi<T>
where
    T: TransactionSend + Clone + 'static,
{
    fn wire<'b>(&self, router: &mut Router) {
        let api = self.clone();
        let post_transaction = move |req: &mut Request| -> IronResult<Response> {
            match req.get::<bodyparser::Struct<MinimalTransactions>>() {
                Ok(Some(transaction)) => {
                    let transaction: Box<Transaction> = transaction.into();
                    let tx_hash = transaction.hash();
                    api.channel.send(transaction).map_err(ApiError::from)?;
                    api.ok_response(&serde_json::to_value(&tx_hash).unwrap())
                }
                Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
            }
        };
        router.post("/v1/transaction", post_transaction, "post_transaction");

        let api = self.clone();
        let get_count = move |_: &mut Request| -> IronResult<Response> {
            let snapshot = api.blockchain.snapshot();
            let schema = DatabaseSchema::new(snapshot);
            let count = schema.counter().get().unwrap_or(0);
            api.ok_response(&serde_json::to_value(&count).unwrap())
        };
        router.get("/v1/count", get_count, "get_count");
    }
}
