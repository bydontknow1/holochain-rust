use error::HolochainError;
use state::State;

/// trait that defines the persistence functionality that holochain_core requires
pub trait Persist {
    fn save(&mut self, state: &State);
    fn load(&self) -> Result<Option<State>, HolochainError>;
}

#[derive(Default, Clone, PartialEq)]
pub struct SimplePersist {
    state: Option<State>,
}

impl Persist for SimplePersist {
    fn save(&mut self, state: &State) {
        self.state = Some(state.clone());
    }
    fn load(&self) -> Result<Option<State>, HolochainError> {
        Ok(self.state.clone())
    }
}

impl SimplePersist {
    pub fn new() -> Self {
        SimplePersist { state: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hash_table::entry::tests::test_entry;
    use snowflake;
    use std::sync::mpsc::channel;

    #[test]
    fn can_instantiate() {
        let store = SimplePersist::new();
        match store.load() {
            Err(_) => assert!(false),
            Ok(state) => match state {
                None => assert!(true),
                _ => assert!(false),
            },
        }
    }

    #[test]
    fn can_roundtrip() {
        let mut store = SimplePersist::new();

        let state = State::new();

        let action = ::state::Action::Agent(::agent::state::Action::Commit {
            entry: test_entry(),
            id: snowflake::ProcessUniqueId::new(),
        });
        let (sender, _receiver) = channel::<::state::ActionWrapper>();
        let (tx_observer, _observer) = channel::<::instance::Observer>();
        let new_state = state.reduce(::state::ActionWrapper::new(action), &sender, &tx_observer);

        store.save(&new_state);

        assert_eq!(store.load().unwrap().unwrap(), new_state);
    }
}