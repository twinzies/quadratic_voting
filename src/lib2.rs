use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display, collections::HashMap};
pub mod proposal;
pub mod voter;
pub(crate) mod tests;
use num::integer::Roots;
pub mod simple_lib;

pub struct Runtime;

impl quadratic_voting::Trait for Runtime {
    type ProposalId = u64;
    type ProposalDescription = String;
    type Weight = u64;
    type AccountId = u64;
    type VoteCount = u64;
    type Time = u64;
    type Currency = u64;
}

pub mod quadratic_voting {
    use std::collections::HashMap;

    use super::*;
    pub trait Trait {
        type Weight: Eq + Copy + Clone;
        type AccountId: Eq;
        type VoteCount: Eq + AddAssign + SubAssign + Add + Sub + Copy + Clone; // todo! Revisit implementation upon introducing balances and currencies.
        type ProposalDescription: Display;
        type Time: PartialOrd + PartialEq + Eq; // primarily needed for comparison to ensure a minimum time for the proposal to be alive is reached.
        type ProposalId: Eq + Display;
        type Currency: PartialOrd + PartialEq + Eq;
    }

    pub enum Errors {
        DispatchError,
        FundsError,
    }
    // Data handling for the module. 
    // todo: What does it mean to declare these types ?
    // todo: How to reference generics here ?
    // todo: Why / how do the generics work here ?
    pub type Proposals <T: Trait> = HashMap<T::ProposalId, proposal::Proposal<T>>;
    pub type VoterInfo <T:Trait> = HashMap<T::ProposalId, Vec<voter::Voter<T>>>;
    pub type QVInternalFunds = u64; // To keep track of funds

    // How to reference a trait bound as a parameter type?
    pub fn create_proposal(fee: <Runtime as Trait>::Currency, who: <Runtime as Trait>::AccountId, proposal_desc:&str) -> Result<(), Errors> {

        let proposal = proposal::Proposal::<Runtime>::new(
            0,
            0,
            who,
            String::from(proposal_desc),
            1234,
        );

        let pId = proposal::generate_pid();
        reserve_funds::<Runtime>(fee, who);
        Ok(())
    }

    pub fn call_proposal<T: Trait>(proposal: T::ProposalId) -> Result<(), Errors> {
        Ok(())
    }

    pub fn cast_vote<T: Trait>() -> Result<(), Errors> {
        Ok(())
    }

    // Module's private functions - ~non dispatchable
    fn count_ballots() -> u64 {
        0
    }

    fn release_funds() -> Result<(), Errors> {
        Ok(())
    }

    fn reserve_funds<T: Trait>(amount: T::Currency, who: T::AccountId) -> Result<(), Errors> {
        Ok(())
    }

    fn cleanup() -> Result<(), Errors> {
        Ok(())
    }

    fn votes_from_fee(fee: u64) -> u64 {
        fee.sqrt() // Quadratic voting
    }

}