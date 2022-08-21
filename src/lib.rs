use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display, collections::HashMap};
extern crate time;

pub mod proposal;
pub mod voter;
pub(crate) mod tests;
use num::integer::Roots;

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

pub struct Runtime;

impl Trait for Runtime {
    type ProposalId = u64;
    type ProposalDescription = String;
    type Weight = u64;
    type AccountId = u64;
    type VoteCount = u64;
    type Time = u64;
    type Currency = u64;
}

pub struct Storage {
    pub all_proposals: HashMap<u64, proposal::Proposal<Runtime>>,

    // Now the generic is a concrete type where the concrete types are explicitly defined l24 - 31.

    pub voter_info: HashMap<u64, Vec<voter::Voter<Runtime>>>,

    pub funds: u64,
}

pub mod quadratic_voting {
    use std::time::SystemTime;

    use crate::{proposal::Proposal, voter::VoteTypes};

    use super::*;

    pub fn create_proposal(storage: &mut Storage, fee: u64, who: u64, proposal_desc:&str) -> Result<(), Errors> {

        let proposal = proposal::Proposal::<Runtime>::new(
            0,
            0,
            who,
            String::from(proposal_desc),
            1234,
        );

        let p_Id = proposal::generate_pid();

        // Add new proposal to storage
        storage.all_proposals.insert(p_Id, proposal);

        // take fee
        storage.funds += fee;

        Ok(())
    }

    pub fn call_proposal<T: Trait<ProposalId = u64>>(storage: &mut Storage, proposal: T::ProposalId) -> Result<(), Errors> {
    
        let proposal = storage.all_proposals.get(&proposal);
        Ok(())
    }

    pub fn cast_vote<T: Trait>(storage: &mut Storage) -> Result<(), Errors> {
        Ok(())
    }

    // Module's private functions - ~non dispatchable

    // Returns winning stance
    fn count_ballots<T: Trait<ProposalId = u64>>(proposal: Proposal<T>) -> VoteTypes {
        match proposal.num_ayes > proposal.num_nays {
            true => VoteTypes::Yay,
            false => if proposal.num_ayes == proposal.num_nays {
                VoteTypes::NoStance
            }
            else {
                VoteTypes::Nay
            }
        }
    }

    fn release_funds(storage: &mut Storage, amount: u64) -> Result<(), Errors> {
        storage.funds -= amount;
        Ok(())
    }

    fn reserve_funds(storage: &mut Storage,amount: u64, who: u64) -> Result<(), Errors> {
        storage.funds += amount;
        Ok(())
    }

    fn proposal_cleanup() -> Result<(), Errors> {
        Ok(())
    }

    fn votes_from_fee(fee: u64) -> u64 {
        fee.sqrt() // Quadratic voting
    }

}