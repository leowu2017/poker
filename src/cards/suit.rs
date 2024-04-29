use super::traits::hash::{ValueHash, ValueHashT};
use super::traits::{order::Order, value::Value};
use std::hash::Hash;

#[derive(Clone, Hash)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Order for Suit {
    fn order(&self) -> u64 {
        match *self {
            Suit::Spade => 0,
            Suit::Heart => 1,
            Suit::Diamond => 2,
            Suit::Club => 3,
        }
    }
}

impl Value for Suit {
    fn value(&self) -> i32 {
        self.order() as i32
    }
}

impl ValueHash for Suit {
    fn value_hash(&self) -> ValueHashT {
        self.value() as ValueHashT
    }
}

pub const ALL_SUIT_NUM: usize = 4;

pub static ALL_SUITS: [Suit; ALL_SUIT_NUM] = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

pub fn all_suits() -> &'static [Suit; ALL_SUIT_NUM] {
    &ALL_SUITS
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suit_order_test() {
        let suit1 = Suit::Spade;
        let suit2 = Suit::Spade;
        assert_eq!(suit1.order(), suit2.order());
        assert_ne!(Suit::Spade.order(), Suit::Heart.order());
        assert_ne!(Suit::Spade.order(), Suit::Diamond.order());
        assert_ne!(Suit::Spade.order(), Suit::Club.order());
    }
    #[test]
    fn suit_value_test() {
        let suit1 = Suit::Spade;
        let suit2 = Suit::Spade;
        assert_eq!(suit1.value(), suit2.value());
        let suit1 = Suit::Heart;
        let suit2 = Suit::Heart;
        assert_eq!(suit1.value(), suit2.value());
    }

    #[test]
    fn suit_hash_test() {
        // value_hash
        let suit1 = Suit::Spade;
        let suit2 = Suit::Spade;
        assert_eq!(suit1.value_hash(), suit2.value_hash());
        let suit1 = Suit::Heart;
        let suit2 = Suit::Heart;
        assert_eq!(suit1.value_hash(), suit2.value_hash());
    }
}
