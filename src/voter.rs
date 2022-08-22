use crate::Trait;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VoteTypes {
    Yay,
    Nay,
    NoStance,
}

#[derive(Debug, PartialEq)]
pub struct Voter<T: Trait> {
    funds: T::Currency,
    who: T::AccountId, 
    stance: VoteTypes,
}

impl <T: Trait> Voter <T> {

    pub fn new(who: T::AccountId, funds:T::Currency, stance: VoteTypes) -> Self {
        Voter {
            funds: funds,
            stance: stance,
            who: who,
        }
    }
    // nice to have
    pub fn update_funds(self, funds: T::Currency) -> Self {
        Voter { 
            funds: funds,
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

    pub fn funds(&self) -> T::Currency{
        self.funds
    }
}