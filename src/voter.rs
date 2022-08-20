use crate::Trait;

pub enum VoteTypes {
    Yay,
    Nay,
}

pub struct Voter<T: Trait> {
    weight: T::Weight,
    who: T::AccountId, 
    stance: VoteTypes
}

impl <T: Trait> Voter <T>{
    pub fn new(who: T::AccountId, weight:T::Weight, stance: VoteTypes) -> Self {
        Voter {
            weight: weight,
            stance: stance,
            who: who,
        }
    }
}