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
    ProposalNotFound,
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
    use std::{time::SystemTime, error::Error};
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

        let p_id = proposal::generate_pid();

        // Add new proposal to storage
        storage.all_proposals.insert(p_id, proposal);

        // take fee
        storage.funds += fee;

        Ok(())
    }

    pub fn call_proposal<T: Trait<ProposalId = u64>>(storage: &mut Storage, proposal: T::ProposalId) -> Result<(), Errors> 
    {
        if storage.all_proposals.get(&proposal).is_some() {
            let win = count_ballots(storage.all_proposals.get(&proposal).unwrap());
            match win.0 {
                VoteTypes::Nay => println!("The Nays have it!"),
                VoteTypes::Yay => println!("The Yays have it!"),
                VoteTypes::NoStance => println!("Tied. No consensus reached.")
            }
            release_funds(storage, win.1); // win[1] represents ballots received
            proposal_cleanup(storage, proposal);
        }
        else {
            return Err(Errors::ProposalNotFound);
        }
        Ok(())
    }

    pub fn cast_vote(storage: &mut Storage) -> Result<(), Errors> {
        Ok(())
    }

    // Module's private functions - ~non dispatchable

    // Returns winning stance
    fn count_ballots<T: Trait<ProposalId = u64>>(proposal: &Proposal<T>) -> (VoteTypes, u64) {
        match proposal.num_ayes > proposal.num_nays {
            true => (VoteTypes::Yay,proposal.num_ayes + proposal.num_nays),
            false => if proposal.num_ayes == proposal.num_nays {
                (VoteTypes::NoStance, proposal.num_ayes + proposal.num_nays)
            }
            else {
                ( VoteTypes::Nay, proposal.num_ayes + proposal.num_nays)
            }
        }
    }

    fn release_funds(storage: &mut Storage, proposal: u64) -> Result<(), Errors> {
        // todo! Release cumulative of weights Storage.voter_info.get(proposal). for each element in the vec<Voters>
        Ok(())
    }

    fn reserve_funds(storage: &mut Storage,amount: u64, who: u64) -> Result<(), Errors> {
        storage.funds += amount;
        Ok(())
    }

    fn proposal_cleanup(storage: &mut Storage, proposal: u64) -> Result<(), Errors> {
        // Delete Proposal id key from storage
        Ok(())
    }

    fn votes_from_fee(fee: u64) -> u64 {
        fee.sqrt() // Quadratic voting
    }

}