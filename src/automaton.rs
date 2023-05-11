//! ## Automaton
//! Les automates à état finis sont des quintuplets A = (X, Q, I, F, S)
//! avec :
//! - X représente un alphaber
//! - Q représente un ensemble d'états
//! - I représente l'état initial
//! - F représente l'ensemble des états finaux
//! - S réprésente la fonction de transition sur Q × X -> Q

use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum State {
    Zeros,
    FirstB,
    SecondB,
    FirstOne,
    Ones,
}

#[derive(Debug)]
pub struct Automaton {
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
            current_state: init_state,
            fstates,
            transition_fn,
        }
    }

    fn get_next_state(&self, c: char) -> Option<&State> {
        self.transition_fn.get(&(self.current_state, c))
    }

    fn switch_state(&mut self, c: char) {
        let next_state = self.get_next_state(c);
        if let None = next_state {
            println!("Le mot n'appartient pas au langage");
            exit(-1);
        }
        self.current_state = next_state.unwrap().to_owned();
    }

    pub fn validate(&mut self, word: &String) -> bool {
        word.chars().for_each(|c| self.switch_state(c));

        self.fstates.contains(&self.current_state)
    }
}
