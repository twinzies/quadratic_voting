use crate::Trait;
use rand::Rng;

pub struct Proposal <T: Trait> {
    pub numAyes: T::VoteCount,
    pub numNays: T::VoteCount,
    pub creator: T::AccountId,
    pub description: T::ProposalDescription,
    pub creationTime: T::Time,
}

pub fn generatePid(proposal: Proposal) -> T::ProposalId {
    let mut rng = rand::thread_rng();
    let iD: u64  = rng.gen_range(15520.0..93932.56);
    return iD
} // Simple ID generation.

impl <T: Trait> Proposal <T> {

    pub fn new(numAyes: T::VoteCount, numNays: T::VoteCount, creator: T::AccountId, description: T::ProposalDescription, creationTime: T::Time) -> Self {
        Proposal {
            numAyes: numAyes,
            numNays: numNays,
            creationTime: creationTime,
            creator: creator,
            description: description,
        }
    }

    pub fn numAyes(self, new_votes: T::VoteCount) -> Self {
        Proposal {
            numAyes: self.numAyes + new_votes,
            ..self
        }
    }

    pub fn numNays(self, new_votes: T::VoteCount) -> Self {
        Proposal {
            numAyes: self.numNays + new_votes,
            ..self
        }
    }

}