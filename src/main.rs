pub mod automaton;

use std::{
    collections::{HashSet, HashMap},
    io
};
use automaton::*;

fn main() -> io::Result<()> {
    let transition_fn = HashMap::from([
        ((State::Zeros, '0'), State::Zeros),
        ((State::Zeros, 'b'), State::FirstB),
        ((State::FirstB, 'b'), State::SecondB),
        ((State::SecondB, '1'), State::FirstOne),
        ((State::FirstOne, '1'), State::Ones),
        ((State::Ones, '1'), State::Ones)
    ]);

    let fstates = HashSet::from([State::Ones]);

    let init_state = State::Zeros;

    let mut automate = Automaton::create(init_state, fstates, transition_fn);

    let mut word = String::new();
    io::stdin().read_line(&mut word)?;
    /* On supprime les espaces et retours à la ligne pour ne pas avoir de problème */
    word = word.trim().to_string(); 
    if automate.validate(&word) {
        println!("Le mot appartient au langage");
    } else {
        println!("Le mot n'appartient pas au langage");
    }
    Ok(())
}
