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
