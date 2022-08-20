use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display};
pub mod proposal;
pub mod voter;
pub(crate) mod tests;

pub mod quadratic_voting {
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
    fn _count_ballots() -> u64 {
        0
    }

    fn _release_funds() -> Result<(), Errors> {
        Ok(())
    }

    fn _reserve_funds() -> Result<(), Errors> {
        Ok(())
    }

    fn _cleanup() -> Result<(), Errors> {
        Ok(())
    }

    fn _votes_from_tokens() -> u64 {
        0
    }



}