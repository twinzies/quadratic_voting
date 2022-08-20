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

}