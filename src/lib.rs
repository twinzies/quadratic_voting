pub mod voter;

pub trait Trait {
    type Weight: Eq;
    type AccountId: Eq;
}