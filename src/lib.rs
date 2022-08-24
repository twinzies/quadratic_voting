use std::{ops::{AddAssign, SubAssign}, fmt::Display, collections::HashMap, hash::Hash};

pub mod proposal;
pub mod voter;
pub(crate) mod tests;
use num::{integer::Roots, CheckedAdd, CheckedSub, Zero, CheckedMul};

pub trait Trait {
    type AccountId: Eq + Display + Hash + Copy;
    type ProposalDescription: Display;
    type ProposalId: Eq + Display + Hash;
    type Currency: PartialOrd + PartialEq + Eq + AddAssign + CheckedAdd + Roots + CheckedSub + Zero + CheckedMul + Copy; // For this scenario Currency is the same units as Currency since the number of votes = squareroot of the deposit which is a Currency.
}

#[derive(PartialEq)]
pub enum Errors {
    DispatchError,
    FundsError,
    ProposalNotFound,
}

pub struct Storage <T: Trait> {
    
    pub all_proposals: HashMap<T::ProposalId, proposal::Proposal<T>>,

    // Now the generic is a concrete type where the concrete types are explicitly defined l24 - 31.

    pub voter_info: HashMap<T::ProposalId, Vec<voter::Voter<T>>>,
    // pub voter_info: HashMap<HashMap<T::AccountId, voter::Voter<T>>>,
    // In Substrate: pub voter_info: HashMap<(T::ProposalId, T::AccountId), voter::Voter<T>>,

    // this is aligned with the storage double map
    pub funds: T::Currency,
}

pub mod quadratic_voting {
    pub type Time = u64;
    use crate::{proposal::Proposal, voter::VoteTypes};
    use super::*;

    // Dispatchable
    // todo! Add release_funds dispatchable function
    pub fn create_proposal <T: Trait<ProposalDescription = String, ProposalId = u64>>(storage: &mut Storage<T>, deposit: T::Currency, who: T::AccountId, proposal_desc:&str) -> Result<u64, Errors> {

        let proposal = proposal::Proposal::<T>::new(
            Zero::zero(), // No votes can be sent upon initialization.
            Zero::zero(),
            who,
            String::from(proposal_desc),
            1234,
        );

        let p_id = proposal::generate_pid();

        // Add new proposal to storage
        storage.all_proposals.insert(p_id, proposal);

        // take deposit
        storage.funds += deposit;

        Ok(p_id)
    }

    // Dispatchable
    pub fn call_proposal<T: Trait<ProposalId = u64>>(storage: &mut Storage<T>, proposal: T::ProposalId) -> Result<VoteTypes, Errors> 
    {
        if storage.all_proposals.get(&proposal).is_some() {
            let win = count_ballots(storage.all_proposals.get(&proposal).unwrap());
            match win.0 {
                VoteTypes::Nay => println!("The Nays have it!"),
                VoteTypes::Yay => println!("The Yays have it!"),
                VoteTypes::NoStance => println!("Tied. No consensus reached."),
                VoteTypes::NoVote => println!("No vote cast"),
            }
            proposal_cleanup(storage, proposal);
            return Ok(win.0);
        }
        else {
            return Err(Errors::ProposalNotFound);
        }
        println!("Proposal ID {:} called and closed.", proposal); // Similar to pallet events.
    }

    // Dispatchable
    // To return deposit on a concluded proposal, an AccountId must invoke the call.
    pub fn return_deposit<T: Trait<ProposalId = u64>>(storage: &mut Storage<T>, who: T::AccountId, proposal: T::ProposalId) {
        release_funds(storage, proposal);
    }

    // Dispatchable
    pub fn cast_vote<T: Trait<ProposalId = u64>>(storage: &mut Storage<T>, who: T::AccountId, number: T::Currency, proposal: T::ProposalId, stance: VoteTypes) -> Result<(), Errors> {

        if storage.all_proposals.contains_key(&proposal) && !storage.all_proposals.get(&proposal).unwrap().voters.contains(&who) {
            //todo! Handle errors
            store_incoming_vote(
                storage, 
                number, 
                stance, 
                proposal, 
                who);
            //todo! Handle errors
            reserve_funds(storage, deposit_from_votes::<T>(number), who);

            println!("Vote successfully cast for {:} by {:}", proposal, who); // ~Emit event
        }
        else {
            return Err(Errors::ProposalNotFound);
        }
        Ok(())
    }

    // Module's private functions - ~non dispatchable

    fn store_incoming_vote <T: Trait<ProposalId = u64>>(storage: &mut Storage<T>, vote_count: T::Currency, stance: VoteTypes, proposal: T::ProposalId, who: T::AccountId) -> Result<(), Errors> {

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
            VoteTypes::NoStance => (),
            VoteTypes::NoVote => (),
        };
        Ok(())
    }

    // Returns winning stance
    fn count_ballots<T: Trait<ProposalId = u64>>(proposal: &Proposal<T>) -> (VoteTypes, T::Currency) {
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

    fn release_funds <T: Trait<ProposalId = u64>>(storage: &mut Storage<T>, proposal: T::ProposalId) -> Result<(), Errors> {
        // todo! Release cumulative of funds Storage.voter_info.get(proposal). for each element in the vec<Voters>
        Ok(())
    }

    fn reserve_funds<T: Trait>(storage: &mut Storage<T>, amount: T::Currency, who: T::AccountId) -> Result<(), Errors> {
        //storage.funds += amount;
        storage.funds = storage.funds.checked_add(&amount).unwrap(); // must do safe math, returns errors in the case of overflow
        // todo! deduct funds from who's Account
        Ok(())
    }

    fn proposal_cleanup<T: Trait>(storage: &mut Storage<T>, proposal: T::ProposalId) -> Result<(), Errors> {
        storage.all_proposals.remove(&proposal);
        storage.voter_info.remove(&proposal); // todo! Error handling required
        Ok(())
    }

    fn deposit_from_votes<T: Trait>(votes: T::Currency) -> T::Currency {
        // Recall that for this (counterintuitive) scenario vote counts are in the same units as Currency. 
        match votes.checked_mul(&votes) {
            None => Zero::zero(),
            Some(deposit) => deposit,
        } // Quadratic voting
    }

}