use super::suit::Suit;
use super::value::Value;

// #[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
#[derive(Eq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Card {
            suit: suit,
            value: value,
        }
    }
}

use std::cmp::PartialEq;
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

use std::cmp::Ordering;
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_test() {
        assert_eq!(
            Card::new(Suit::Spade, Value::Ace),
            Card::new(Suit::Spade, Value::Ace)
        );
        assert_ne!(
            Card::new(Suit::Spade, Value::Ace),
            Card::new(Suit::Spade, Value::Two)
        );
        assert!(Card::new(Suit::Spade, Value::Ace) == Card::new(Suit::Spade, Value::Ace));
        assert!(Card::new(Suit::Spade, Value::Ace) == Card::new(Suit::Heart, Value::Ace));
        assert!(Card::new(Suit::Spade, Value::Ace) != Card::new(Suit::Spade, Value::King));
        assert!(Card::new(Suit::Spade, Value::Ace) > Card::new(Suit::Spade, Value::King));
        assert!(Card::new(Suit::Spade, Value::King) < Card::new(Suit::Spade, Value::Ace));
    }
}
