pub mod automaton;

use std::{
    collections::{HashSet, HashMap},
    io::{self, Write}
};
use automaton::*;

fn main() -> io::Result<()> {
    let transition_fn = HashMap::from([
        ((State::S, '0'), State::S),
        ((State::S, 'b'), State::A),
        ((State::A, 'b'), State::B),
        ((State::B, '1'), State::C),
        ((State::C, '1'), State::D),
        ((State::D, '1'), State::D)
        ]);

    let fstates = HashSet::from([State::D]);

    let init_state = State::Zeros;

    let mut automate = Automaton::create(init_state, fstates, transition_fn);

    let mut word = String::new();
    loop {
        print!("Entrez un mot : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut word)?;
        /* On supprime les espaces et retours à la ligne pour ne pas avoir de problème */
        word = word.trim().to_string(); 
        if automate.validate(&word) {
            println!("Le mot appartient au langage");
        } else {
            println!("Le mot n'appartient pas au langage");
        }
        automate.reset();
        word.clear();
    }
    Ok(())
}
