//! ## Automaton
//! Les automates à état finis sont des quintuplets A = (X, Q, I, F, S)
//! avec :
//! - X représente un alphaber
//! - Q représente un ensemble d'états
//! - I représente l'état initial
//! - F représente l'ensemble des états finaux
//! - S réprésente la fonction de transition sur Q × X -> Q

use std::{
    collections::{HashMap, HashSet}
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum State {
    S,
    A,
    B,
    C,
    D,
}

#[derive(Debug)]
pub struct Automaton {
    init_state: State,
    current_state: State,
    fstates: HashSet<State>,
    transition_fn: HashMap<(State, char), State>,
}

impl Automaton {
    pub fn create(
        init_state: State,
        fstates: HashSet<State>,
        transition_fn: HashMap<(State, char), State>,
    ) -> Automaton {
        Automaton {
            init_state,
            current_state: init_state,
            fstates,
            transition_fn,
        }
    }

    fn get_next_state(&self, c: char) -> Option<&State> {
        self.transition_fn.get(&(self.current_state, c))
    }

    fn switch_state(&mut self, c: char) -> Result<(), ()> {
        let next_state = self.get_next_state(c);
        if let None = next_state {
            return Err(());
        }
        self.current_state = next_state.unwrap().to_owned();
        Ok(())
    }

    pub fn validate(&mut self, word: &String) -> bool {
        for c in word.chars() {
            if let Err(_) = self.switch_state(c) {
                return false;
            }
        }
        self.fstates.contains(&self.current_state)
    }
    pub fn reset(&mut self) {
        self.current_state = self.init_state;
    }
}
