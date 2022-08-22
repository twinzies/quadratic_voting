use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display, collections::HashMap};
#[cfg(test)]
pub mod tests {
    use crate::{Trait, Storage};
    use crate::{proposal, voter};
    use crate::voter::{VoteTypes, Voter};
    use super::*;

    #[derive(PartialEq, Debug)]
    pub struct Test;

    impl Trait for Test {
        type Weight = u64;
        type AccountId = u64;
        type VoteCount = u64;
        type ProposalDescription =  String;
        type Time = u64; // primarily needed for comparison to ensure a minimum time for the proposal to be alive is reached.
        type ProposalId = u64;
        type Currency = u64;
    }

    mod T {
    }
    #[test]
    fn test_proposal() {
        // unit tests for proposal
        let user1: u64 = 41;
        let mut voters:Vec<<Test as Trait>::AccountId> = Vec::new();

        let testproposal = proposal::Proposal::<Test> {
            num_ayes: 0,
            num_nays: 0,
            creator: user1, 
            description: String::from("test proposal"),
            creation_time: 1661004817,
            voters,
        };

        let mut myproposal = 
        proposal::Proposal::new (
            0,
            0,
            user1, 
            String::from("test proposal"),
            1661004817);

        assert_eq!(testproposal, myproposal);

        myproposal = myproposal.mod_ayes(2);
        assert_ne!(myproposal, testproposal);

    }
    
    #[test]
    fn test_voter() {
        // unit tests for voter

        let account1: u64 = 41;
        let numVotes:u64 = 5;
        let weight = numVotes * numVotes;
        let stance = VoteTypes::Nay;

        // Note that the generic is required to take an explicit form here at the time of the binding.

        let voter1: Voter<Test> = voter::Voter::new(
            account1,
            weight,
            stance);
        
        assert_eq!(voter1.stance(), VoteTypes::Nay);

        assert_ne!(voter1.weight(), 5);
    }

    #[test]
    fn test_quadratic_voting() {
        // instantiate a Storage instance with an empty hashmap.
        
        let mut store = Storage {
            all_proposals: todo!(),
            voter_info: todo!(),
            funds: todo!(),
        };
    }

}
