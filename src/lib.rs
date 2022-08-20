use std::{ops::{AddAssign, SubAssign}, fmt::Display};

pub mod voter;

pub trait Trait {
    type Weight: Eq;
    type AccountId: Eq;
    type VoteCount: Eq + AddAssign + SubAssign;
    type ProposalDescription: Display;
    type Time: PartialOrd + PartialEq + Eq; // primarily needed for comparison to ensure a minimum time for the proposal to be alive is reached.
    type ProposalId: Eq + Display;
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
