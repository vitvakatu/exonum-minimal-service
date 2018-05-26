use exonum::storage::{Entry, Fork, Snapshot};

/// Database schema for `MinimalService`.
pub struct DatabaseSchema<T> {
    view: T,
}

impl<T> DatabaseSchema<T>
where
    T: AsRef<Snapshot>,
{
    pub fn new(view: T) -> Self {
        Self { view }
    }

    /// Immutable access to counter.
    pub fn counter(&self) -> Entry<&Snapshot, i32> {
        Entry::new("minimal.counter", self.view.as_ref())
    }
}

impl<'a> DatabaseSchema<&'a mut Fork> {
    /// Mutable access to counter.
    pub fn counter_mut(&mut self) -> Entry<&mut Fork, i32> {
        Entry::new("minimal.counter", self.view)
    }
}
