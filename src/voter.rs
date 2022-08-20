use crate::quadratic_voting::Trait;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    // nice to have
    pub fn update_weight(self, weight: T::Weight) -> Self {
        Voter { 
            weight: weight,
            ..self
        }
    }
    // nice to have
    pub fn update_stance(self, stance: VoteTypes) -> Self {
        Voter { 
            stance: stance,
            ..self
        }
    }

    pub fn stance(&self) -> VoteTypes {
        self.stance
    }

    pub fn weight(&self) -> T::Weight{
        self.weight
    }
}