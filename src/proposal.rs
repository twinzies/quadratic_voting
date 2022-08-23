use std::ops::Add;

use crate::{Trait, quadratic_voting};
use quadratic_voting::Time as Time;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Proposal <T: Trait> {

    pub num_ayes: T::Currency,
    pub num_nays: T::Currency,
    pub creator: T::AccountId,
    pub description: T::ProposalDescription,
    pub creation_time: Time,
    pub voters: Vec<T::AccountId>, // Don't need since I can iterate over all keys.
}

pub fn generate_pid () -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(15520..93932)
} // Simple ID generation.

impl <T: Trait> Proposal <T> {

    pub fn new(num_ayes: T::Currency, num_nays: T::Currency, creator: T::AccountId, description: T::ProposalDescription, creation_time: Time) -> Self {
        Proposal {
            num_ayes: num_ayes,
            num_nays: num_nays,
            creator: creator,
            description: description,
            creation_time: creation_time,
            voters: Vec::new(),
        }
    }

    pub fn mod_ayes(self, new_votes: T::Currency) -> Self {
        Proposal {
            num_ayes: self.num_ayes + new_votes,
            ..self
        }
    }

    pub fn mod_nays(self, new_votes: T::Currency) -> Self {
        Proposal {
            num_ayes: self.num_nays + new_votes,
            ..self
        }
    }

}