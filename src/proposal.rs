use std::ops::Add;

use crate::Trait;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Proposal <T: Trait> {
    pub numAyes: u64,
    pub numNays: u64,
    pub creator: T::AccountId,
    pub description: T::ProposalDescription,
    pub creationTime: T::Time,
}

pub fn generatePid <T: Trait> () -> u64 {
    let mut rng = rand::thread_rng();
    let iD: u64 = rng.gen_range(15520..93932);
    return iD;
} // Simple ID generation.

impl <T: Trait> Proposal <T> {

    pub fn new(numAyes: u64, numNays: u64, creator: T::AccountId, description: T::ProposalDescription, creationTime: T::Time) -> Self {
        Proposal {
            numAyes: numAyes,
            numNays: numNays,
            creationTime: creationTime,
            creator: creator,
            description: description,
        }
    }

    pub fn modAyes(self, new_votes: u64) -> Self {
        Proposal {
            numAyes: self.numAyes + new_votes,
            ..self
        }
    }

    pub fn modNays(self, new_votes: u64) -> Self {
        Proposal {
            numAyes: self.numNays + new_votes,
            ..self
        }
    }

}