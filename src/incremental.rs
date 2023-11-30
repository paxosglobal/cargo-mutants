// Copyright 2023 Paxos

//! Logic for incremantal runs
use crate::{
    mutate::{Mutant, MutantHash},
    output::{PositiveOutcome, OUTDIR_NAME, POSITIVE_OUTCOMES_FILE},
};
use anyhow::{anyhow, Result};
use std::{collections::HashSet, fs, path::PathBuf};

pub fn filter(mutants: Vec<Mutant>) -> (Option<Vec<PositiveOutcome>>, Vec<Mutant>) {
    // TODO: add logging here for error cases
    let last_positive_outcomes = match read_last_positive_outcomes() {
        Ok(outcomes) => Some(outcomes),
        Err(_) => None,
    };
    // if last_positive_outcomes is none the hash set will be empty thereby allowing
    let existing_mutants: HashSet<MutantHash> = last_positive_outcomes
        .iter()
        .flatten()
        .map(|o| o.mutant_hash())
        .collect();
    let mutants = mutants
        .into_iter()
        .filter(|m| !existing_mutants.contains(&m.calculate_hash()))
        .collect();
    (last_positive_outcomes, mutants)
}

fn read_last_positive_outcomes() -> Result<Vec<PositiveOutcome>> {
    // if the positive_outcomes file exists, read it and return the deserialized vector of PositiveOutcome
    // else return None
    // TODO: Add in_dir here to support user specified output directories
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        OUTDIR_NAME,
        POSITIVE_OUTCOMES_FILE,
    ]
    .iter()
    .collect();
    let json_str = fs::read_to_string(path.clone());
    match json_str {
        Ok(contents) => serde_json::from_str(&contents).map_err(|e| anyhow!("{}", e)),
        Err(_) => Err(anyhow!("Failed to read file at path: {}", path.display())),
    }
}
