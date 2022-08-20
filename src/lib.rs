use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display};
pub mod proposal;
pub mod voter;
pub(crate) mod tests;

pub type ProposalId = u64;

pub mod quadratic_voting {
    use std::collections::HashMap;

    use super::*;
    pub trait Trait {
        type Weight: Eq;
        type AccountId: Eq;
        type VoteCount: Eq + AddAssign + SubAssign + Add + Sub + Copy + Clone; // todo! Revisit implementation upon introducing balances and currencies.
        type ProposalDescription: Display;
        type Time: PartialOrd + PartialEq + Eq; // primarily needed for comparison to ensure a minimum time for the proposal to be alive is reached.
        type ProposalId: Eq + Display;
    }

    pub enum Errors {
        DispatchError,
        FundsError,
    }
    // Data handling for the module. 
    // todo: What does it mean to declare these types ?
    // todo: How to reference generics here ?
    // todo: Why / how do the generics work here ?
    pub type Proposals <T> = HashMap<ProposalId, proposal::Proposal<T>>;
    pub type VoterInfo <T> = HashMap<ProposalId, Vec<voter::Voter<T>>>;

    pub fn create_proposal() -> Result<(), Errors> {
        Ok(())
    }

    pub fn call_proposal() -> Result<(), Errors> {
        Ok(())
    }

    pub fn cast_vote() -> Result<(), Errors> {
        Ok(())
    }

    // Module's private functions - ~non dispatchable
    fn count_ballots() -> u64 {
        0
    }

    fn release_funds() -> Result<(), Errors> {
        Ok(())
    }

    fn reserve_funds() -> Result<(), Errors> {
        Ok(())
    }

    fn cleanup() -> Result<(), Errors> {
        Ok(())
    }

    fn votes_from_tokens() -> u64 {
        0
    }



}