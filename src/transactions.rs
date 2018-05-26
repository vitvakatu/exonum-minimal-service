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

use exonum::blockchain::ExecutionError;
use exonum::blockchain::Transaction;
use exonum::storage::Fork;

use schema::DatabaseSchema;

transactions!{
    pub MinimalTransactions {
        const SERVICE_ID = 0;

        struct Increase {
            amount: u16,
            seed: u32,
        }

        struct Decrease {
            amount: u16,
            seed: u32,
        }
    }
}

impl Transaction for Increase {
    fn verify(&self) -> bool {
        true
    }

    fn execute(&self, fork: &mut Fork) -> Result<(), ExecutionError> {
        let mut schema = DatabaseSchema::new(fork);
        let count = schema.counter().get().unwrap_or(0);
        let new_count = count + self.amount() as i32;
        schema.counter_mut().set(new_count);
        Ok(())
    }
}

impl Transaction for Decrease {
    fn verify(&self) -> bool {
        true
    }

    fn execute(&self, fork: &mut Fork) -> Result<(), ExecutionError> {
        let mut schema = DatabaseSchema::new(fork);
        let count = schema.counter().get().unwrap_or(0);
        let new_count = count - self.amount() as i32;
        schema.counter_mut().set(new_count);
        Ok(())
    }
}
