use super::traits::hash::{ValueHash, ValueHashT};
use super::traits::{order::Order, value::Value};
use std::hash::Hash;

#[derive(Clone, Hash)]
pub enum Number {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Order for Number {
    fn order(&self) -> u64 {
        match *self {
            Number::Ace => 0,
            Number::Two => 1,
            Number::Three => 2,
            Number::Four => 3,
            Number::Five => 4,
            Number::Six => 5,
            Number::Seven => 6,
            Number::Eight => 7,
            Number::Nine => 8,
            Number::Ten => 9,
            Number::Jack => 10,
            Number::Queen => 11,
            Number::King => 12,
        }
    }
}

impl Value for Number {
    fn value(&self) -> i32 {
        self.order() as i32
    }
}

impl ValueHash for Number {
    fn value_hash(&self) -> ValueHashT {
        self.value() as ValueHashT
    }
}

pub const ALL_NUMBER_NUM: usize = 13;

pub static ALL_NUMBERS: [Number; ALL_NUMBER_NUM] = [
    Number::Ace,
    Number::Two,
    Number::Three,
    Number::Four,
    Number::Five,
    Number::Six,
    Number::Seven,
    Number::Eight,
    Number::Nine,
    Number::Ten,
    Number::Jack,
    Number::Queen,
    Number::King,
];

pub fn all_numbers() -> &'static [Number; ALL_NUMBER_NUM] {
    &ALL_NUMBERS
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_order_test() {
        assert_eq!(Number::Two.order(), Number::Two.order());
        assert_ne!(Number::Two.order(), Number::Three.order());
        assert!(Number::Three.order() != Number::Four.order());
        assert!(Number::King.order() != Number::Ten.order());
    }
    #[test]
    fn number_value_test() {
        let num1 = Number::Ace;
        let num2 = Number::Ace;
        assert_eq!(num1.value(), num2.value());
        let num1 = Number::Two;
        let num2 = Number::Two;
        assert_eq!(num1.value(), num2.value());
    }

    #[test]
    fn number_hash_test() {
        // value_hash
        let num1 = Number::Ace;
        let num2 = Number::Ace;
        assert_eq!(num1.value_hash(), num2.value_hash());
        let num1 = Number::Two;
        let num2 = Number::Two;
        assert_eq!(num1.value_hash(), num2.value_hash());
    }
}
