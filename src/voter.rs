pub enum VoteTypes {
    Yay,
    Nay,
}

pub trait Trait {
    type Weight: Eq;
    type AccountId: Eq;
}

pub struct Voter<T: Trait> {
    weight: T::Weight,
    who: T::AccountId, 
    stance: VoteTypes
}

impl Voter {
    pub fn new(who: T::AccountId, weight:T::Weight, stance: VoteTypes) -> Self {
        Voter {
            weight: weight,
            address: address,
            stance: stance,
        }
    }
}