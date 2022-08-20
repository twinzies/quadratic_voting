use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display};pub mod proposal;
pub mod voter;

pub trait Trait {
    type Weight: Eq;
    type AccountId: Eq;
    type VoteCount: Eq + AddAssign + SubAssign + Add + Sub + Copy + Clone; // todo! Revisit implementation upon introducing balances and currencies.
    type ProposalDescription: Display;
    type Time: PartialOrd + PartialEq + Eq; // primarily needed for comparison to ensure a minimum time for the proposal to be alive is reached.
    type ProposalId: Eq + Display;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    pub struct Test;

    impl Trait for Test {
        type Weight = u64;
        type AccountId = u64;
        type VoteCount = u64;
        type ProposalDescription =  String;
        type Time = u64; // primarily needed for comparison to ensure a minimum time for the proposal to be alive is reached.
        type ProposalId = u64; // assume epoch
    }

    #[test]
    fn Proposal() {
        // unit tests for proposal
        let user1: u64 = 41;

        let testproposal = proposal::Proposal::<Test> {
            numAyes: 0,
            numNays: 0,
            creator: user1, 
            description: String::from("test proposal"),
            creationTime: 1661004817,
        };

        let mut myproposal = 
        proposal::Proposal::new(
            0,
            0,
            user1, 
            String::from("test proposal"),
            1661004817,);

        assert_eq!(testproposal, myproposal);

        myproposal = myproposal.modAyes(2);
        assert_ne!(myproposal, testproposal);

        }

}
