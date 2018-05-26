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
