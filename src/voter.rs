use crate::Trait;

        #[derive(Debug, PartialEq)]

pub enum VoteTypes {
    Yay,
    Nay,
}

#[derive(Debug, PartialEq)]
pub struct Voter<T: Trait> {
    weight: T::Weight,
    who: T::AccountId, 
    stance: VoteTypes,
}

impl <T: Trait> Voter <T> {

    pub fn new(who: T::AccountId, weight:T::Weight, stance: VoteTypes) -> Self {
        Voter {
            weight: weight,
            stance: stance,
            who: who,
        }
    }

    pub fn weight(self, weight: T::Weight) -> Self {
        Voter { 
            weight: weight,
            ..self
        }
    }

    pub fn stance(self, stance: VoteTypes) -> Self {
        Voter { 
            stance: stance,
            ..self
        }
    }
}