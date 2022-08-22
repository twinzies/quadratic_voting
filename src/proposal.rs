use std::ops::Add;

use crate::Trait;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Proposal <T: Trait> {
    pub num_ayes: T::VoteCount,
    pub num_nays: T::VoteCount,
    pub creator: T::AccountId,
    pub description: T::ProposalDescription,
    pub creation_time: T::Time,
    pub voters: Vec<T::AccountId>,
}

pub fn generate_pid () -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(15520..93932)
} // Simple ID generation.

impl <T: Trait> Proposal <T> {

    pub fn new(num_ayes: T::VoteCount, num_nays: T::VoteCount, creator: T::AccountId, description: T::ProposalDescription, creation_time: T::Time) -> Self {
        Proposal {
            num_ayes: num_ayes,
            num_nays: num_nays,
            creator: creator,
            description: description,
            creation_time: creation_time,
            voters: Vec::new(),
        }
    }

    pub fn mod_ayes(self, new_votes: T::VoteCount) -> Self {
        Proposal {
            num_ayes: self.num_ayes + new_votes,
            ..self
        }
    }

    pub fn mod_nays(self, new_votes: T::VoteCount) -> Self {
        Proposal {
            num_ayes: self.num_nays + new_votes,
            ..self
        }
    }

}