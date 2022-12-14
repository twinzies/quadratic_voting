use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display, collections::HashMap};
use crate::Errors;

#[cfg(test)]
pub mod tests {
    use std::io::Error;

    use crate::{Trait, Storage, quadratic_voting};
    use crate::{proposal, voter};
    use crate::voter::{VoteTypes, Voter};
    use super::*;

    #[derive(PartialEq, Debug)]
    pub struct Test;

    impl Trait for Test {
        type Currency = u64;
        type AccountId = u64;
        type ProposalDescription =  String;
        type ProposalId = u64;
    }

    mod test {
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
        let Currency = numVotes * numVotes;
        let stance = VoteTypes::Nay;

        // Note that the generic is required to take an explicit form here at the time of the binding.

        let voter1: Voter<Test> = voter::Voter::new(
            account1,
            Currency,
            stance);
        
        assert_eq!(voter1.stance(), VoteTypes::Nay);

        assert_ne!(voter1.funds(), 5);
    }

    #[test]
    fn test_quadratic_voting() {

        // todo! Add tests for pub mod quadratic voting.
        let mut test_proposals: HashMap<u64, proposal::Proposal<Test>> = HashMap::new();
        let mut test_voter_info: HashMap<u64, Vec<voter::Voter<Test>>> = HashMap::new();

        let mut teststore = Storage {
            all_proposals: test_proposals,
            voter_info: test_voter_info,
            funds: 0,
        };

        let account1: u64 = 41;
        let account2: u64 = 42;        
        
        
        let test_proposal = quadratic_voting::create_proposal(&mut teststore, 100, account1, "Testing to be allowed!").unwrap_or_default();

        // The range of valid proposal Ids
        let range = 15520..93932;

        // Test proposal creation
        assert_ne!(test_proposal, 0);
        assert!(range.contains(&test_proposal));

        // Cast vote to junk proposal
        let test_vote = quadratic_voting::cast_vote(&mut teststore, account2, 5, test_proposal + 5, VoteTypes::Nay);

        // Test cast vote
        assert!(test_vote.is_err());

        // Test call proposal

    }

}
