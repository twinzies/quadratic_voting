use std::{ops::{AddAssign, SubAssign, Sub, Add}, fmt::Display, collections::HashMap, hash::Hash};

pub mod proposal;
pub mod voter;
pub(crate) mod tests;
use num::integer::Roots;

pub trait Trait {
    type Weight: Eq + Copy + Clone;
    type AccountId: Eq + Display;
    type VoteCount: Eq + AddAssign + SubAssign + Add + Sub + Copy + Clone
    + PartialOrd + PartialEq; // todo! Revisit implementation upon introducing balances and currencies.
    type ProposalDescription: Display;
    type Time: PartialOrd + PartialEq + Eq; // primarily needed for comparison to ensure a minimum time for the proposal to be alive is reached.
    type ProposalId: Eq + Display + Hash;
    type Currency: PartialOrd + PartialEq + Eq;
}

pub enum Errors {
    DispatchError,
    FundsError,
    ProposalNotFound,
}

pub struct Storage <T: Trait> {
    pub all_proposals: HashMap<T::ProposalId, proposal::Proposal<T>>,

    // Now the generic is a concrete type where the concrete types are explicitly defined l24 - 31.

    pub voter_info: HashMap<T::ProposalId, Vec<voter::Voter<T>>>,

    pub funds: u64,
}

pub mod quadratic_voting {
    use crate::{proposal::Proposal, voter::VoteTypes};
    use super::*;

    // Dispatchable
    pub fn create_proposal <T: Trait<ProposalDescription = String>>(storage: &mut Storage<T>, fee: T::Currency, who: T::AccountId, proposal_desc:&str) -> Result<(), Errors> {

        let proposal = proposal::Proposal::<T>::new(
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

    // Dispatchable
    pub fn call_proposal<T: Trait<ProposalId = u64>>(storage: &mut Storage<T>, proposal: T::ProposalId) -> Result<(), Errors> 
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
        println!("Proposal ID {:} called and closed.", proposal); // Similar to pallet events.
        Ok(())
    }

    // Dispatchable
    pub fn cast_vote<T: Trait>(storage: &mut Storage<T>, who: T::AccountId, currency: T::Currency, proposal: T::ProposalId, stance: VoteTypes) -> Result<(), Errors> {

        if storage.all_proposals.contains_key(&proposal) && !storage.all_proposals.get(&proposal).unwrap().voters.contains(&who) {
            //todo! Handle errors
            store_incoming_vote(
                storage, 
                votes_from_fee(currency), 
                stance, 
                proposal, 
                who);
            //todo! Handle errors
            reserve_funds(storage, currency, who);

            println!("Vote successfully cast for {:} by {:}", proposal, who); // ~Emit event
        }
        else {
            return Err(Errors::ProposalNotFound);
        }
        Ok(())
    }

    // Module's private functions - ~non dispatchable

    fn store_incoming_vote <T: Trait>(storage: &mut Storage<T>, vote_count: T::VoteCount, stance: VoteTypes, proposal: T::ProposalId, who: T::AccountId) -> Result<(), Errors> {

        let vote_caster = voter::Voter::<T>::new(
            who,
            vote_count,
            stance,
        );
        storage.voter_info.get_mut(&proposal).unwrap().push(vote_caster);
        let cur_proposal = storage.all_proposals.get_mut(&proposal).unwrap();
        let new_count = match stance {
            VoteTypes::Nay => cur_proposal.num_nays += vote_count,
            VoteTypes::Yay => cur_proposal.num_ayes += vote_count,
            VoteTypes::NoStance => ()
        };
        Ok(())
    }

    // Returns winning stance
    fn count_ballots<T: Trait<ProposalId = u64>>(proposal: &Proposal<T>) -> (VoteTypes, u64) {
        match proposal.num_ayes > proposal.num_nays {
            true => (VoteTypes::Yay, proposal.num_ayes + proposal.num_nays),
            false => if proposal.num_ayes == proposal.num_nays {
                (VoteTypes::NoStance, proposal.num_ayes + proposal.num_nays)
            }
            else {
                ( VoteTypes::Nay, proposal.num_ayes + proposal.num_nays)
            }
        }
    }

    fn release_funds <T: Trait>(storage: &mut Storage<T>, proposal: u64) -> Result<(), Errors> {
        // todo! Release cumulative of weights Storage.voter_info.get(proposal). for each element in the vec<Voters>
        Ok(())
    }

    fn reserve_funds<T:Trait>(storage: &mut Storage<T>,amount: T::Currency, who: T::AccountId) -> Result<(), Errors> {
        storage.funds += amount;
        // todo! deduct funds from who's Account
        Ok(())
    }

    fn proposal_cleanup<T: Trait>(storage: &mut Storage<T>, proposal: T::ProposalId) -> Result<(), Errors> {
        storage.all_proposals.remove(&proposal);
        storage.voter_info.remove(&proposal); // todo! Error handling required
        Ok(())
    }

    fn votes_from_fee<T: Trait>(fee: T::Currency) -> T::VoteCount {
        fee.sqrt() // Quadratic voting
    }

}