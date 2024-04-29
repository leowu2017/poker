use std::hash::{DefaultHasher, Hash, Hasher};

use super::number::{all_numbers, Number, ALL_NUMBER_NUM};
use crate::cards::suit::{all_suits, Suit, ALL_SUIT_NUM};
use crate::cards::traits::hash::{UniqueHash, UniqueHashT, ValueHash, ValueHashT};
use crate::cards::traits::value::Value;

pub const ALL_CARDS_NUM: usize = ALL_SUIT_NUM * ALL_NUMBER_NUM;

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub number: Number,
    unique_hash: UniqueHashT,
}

impl Card {
    pub fn new(suit: Suit, number: Number) -> Self {
        Card {
            suit: suit,
            number: number,
            unique_hash: Self::calculate_unique_hash(),
        }
    }
}

impl Value for Card {
    fn value(&self) -> i32 {
        self.number.value()
    }
}

impl ValueHash for Card {
    fn value_hash(&self) -> ValueHashT {
        let mut hasher = DefaultHasher::new();
        self.suit.hash(&mut hasher);
        self.number.hash(&mut hasher);
        hasher.finish() as ValueHashT
    }
}

impl UniqueHash for Card {
    fn set_unique_hash(&mut self, hash: UniqueHashT) {
        self.unique_hash = hash;
    }
    fn unique_hash(&self) -> UniqueHashT {
        self.unique_hash
    }
}

pub fn all_cards() -> &'static [Card; ALL_CARDS_NUM] {
    lazy_static! {
        static ref ALL_CARDS: [Card; ALL_CARDS_NUM] = {
            let all_numbers = all_numbers();
            let all_suits = all_suits();
            core::array::from_fn(|idx: usize| {
                let suit_idx = idx / ALL_NUMBER_NUM;
                let number_idx = idx % ALL_NUMBER_NUM;
                Card::new(all_suits[suit_idx].clone(), all_numbers[number_idx].clone())
            })
        };
    }
    &ALL_CARDS
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn card_value_test() {
        let card1 = Card::new(Suit::Spade, Number::Ace);
        let card2 = Card::new(Suit::Spade, Number::Ace);
        assert_eq!(card1.value(), card2.value());
        let card1 = Card::new(Suit::Spade, Number::Ace);
        let card2 = Card::new(Suit::Heart, Number::Ace);
        assert_eq!(card1.value(), card2.value());
        let card1 = Card::new(Suit::Spade, Number::Ace);
        let card2 = Card::new(Suit::Spade, Number::Two);
        assert_ne!(card1.value(), card2.value());
    }

    #[test]
    fn card_hash_test() {
        // value_hash
        let card1 = Card::new(Suit::Spade, Number::Ace);
        let card2 = Card::new(Suit::Spade, Number::Ace);
        assert_eq!(card1.value_hash(), card2.value_hash());
        let card1 = Card::new(Suit::Spade, Number::Ace);
        let card2 = Card::new(Suit::Heart, Number::Ace);
        assert_ne!(card1.value_hash(), card2.value_hash());
        // unique_hash
        let card1 = Card::new(Suit::Spade, Number::Ace);
        let card2 = Card::new(Suit::Spade, Number::Ace);
        assert_ne!(card1.unique_hash(), card2.unique_hash());
        let card1 = Card::new(Suit::Spade, Number::Ace);
        let card2 = Card::new(Suit::Heart, Number::Ace);
        assert_ne!(card1.unique_hash(), card2.unique_hash());
    }

    #[test]
    fn card_unique_hash_test() {
        // Every card has unique value.
        let mut hs = HashSet::new();
        let all_cards = all_cards();
        for card in all_cards {
            hs.insert(card.unique_hash());
        }
        assert_eq!(hs.len(), ALL_CARDS_NUM);
    }
}
