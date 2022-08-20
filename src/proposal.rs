use crate::quadratic_voting::Trait;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Proposal <T: Trait> {
    pub num_ayes: u64,
    pub num_nays: u64,
    pub creator: T::AccountId,
    pub description: T::ProposalDescription,
    pub creation_time: T::Time,
}

pub fn generate_pid <T: Trait> () -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(15520..93932)
} // Simple ID generation.

impl <T: Trait> Proposal <T> {

    pub fn new(num_ayes: u64, num_nays: u64, creator: T::AccountId, description: T::ProposalDescription, creation_time: T::Time) -> Self {
        Proposal {
            num_ayes: num_ayes,
            num_nays: num_nays,
            creation_time: creation_time,
            creator: creator,
            description: description,
        }
    }

    pub fn mod_ayes(self, new_votes: u64) -> Self {
        Proposal {
            num_ayes: self.num_ayes + new_votes,
            ..self
        }
    }

    pub fn mod_nays(self, new_votes: u64) -> Self {
        Proposal {
            num_ayes: self.num_nays + new_votes,
            ..self
        }
    }

}