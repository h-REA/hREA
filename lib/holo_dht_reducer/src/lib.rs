/**
 * A Redux-like store which maintains a list of side-effect operations to apply
 * to the Holochain DHT in addition to its internal state.
 *
 * This allows multiple actions to be merged into the smallest possible set of
 * DHT update operations and thus minimise network updates.
 */

mod dht_operations;

use dht_operations::DHTOperations;

pub struct Store<T: Clone, U> {
    state: T,
    ops: DHTOperations,
    reducer: fn(&T, &DHTOperations, U) -> (T, DHTOperations),
}

impl<T: Clone, U> Store<T, U> {
    pub fn create_store(reducer: fn(&T, &DHTOperations, U) -> (T, DHTOperations), initial_state: T) -> Store<T, U> {
        Store {
            state: initial_state,
            ops: DHTOperations::default(),
            reducer: reducer,
        }
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }

    pub fn get_operations(&self) -> &DHTOperations {
        &self.ops
    }

    pub fn dispatch(&mut self, action: U) {
        let (state, ops) = (self.reducer)(&self.state, &self.ops, action);
        self.state = state;
        self.ops = ops;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
