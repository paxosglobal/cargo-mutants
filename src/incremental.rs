// Copyright 2023 Paxos

//! Logic for incremantal runs

use crate::{mutate::Mutant, output::PositiveOutcomes};

pub fn filter(mutants: Vec<Mutant>) -> (Option<PositiveOutcomes>, Vec<Mutant>) {
    let last_positive_outcomes = read_last_positive_outcomes();
    // TODO Exclude mutants whose hash is in the last positive outcomes
    let mutants = mutants.into_iter().filter(|m| todo!());

    (last_positive_outcomes, mutants.collect())
}

fn read_last_positive_outcomes() -> Option<PositiveOutcomes> {
    todo!()
}
