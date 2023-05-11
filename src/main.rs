pub mod automaton;

use std::collections::{HashSet, HashMap};

use automaton::*;

fn main() {
    let transition_fn = HashMap::from([
        ((State::Zeros, '0'), State::Zeros),
        ((State::Zeros, 'b'), State::FirstB),
        ((State::FirstB, 'b'), State::SecondB),
        ((State::SecondB, '1'), State::Ones),
        ((State::Ones, '1'), State::Ones)
    ]);

    let fstates = HashSet::from([State::Ones]);

    let init_state = State::Zeros;

    let mut automate = Automaton::create(init_state, fstates, transition_fn);

    let w = String::from("0bb1");

    if automate.validate(&w) {
        println!("Le mot appartient au langage");
    } else {
        println!("Le mot n'appartient pas au langage");
    }
}
