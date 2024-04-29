use super::card::{all_cards, Card, ALL_CARDS_NUM};
use super::traits::deck::Deck as DeckTrait;

#[derive(Clone)]
pub struct Deck {
    pub cards: [Card; ALL_CARDS_NUM],
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: all_cards().clone(),
        }
    }
}

impl DeckTrait for Deck {
    fn card_num(&self) -> usize {
        self.cards.len()
    }
    fn swap(&mut self, idx1: usize, idx2: usize) {
        self.cards.swap(idx1, idx2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::traits::hash::{UniqueHash, ValueHash};
    use std::collections::HashSet;

    #[test]
    fn deck_shuffle_test() {
        // card uniqueness
        let mut deck = Deck::new();
        let mut hs_value_hash = HashSet::new();
        let mut hs_unique_hash = HashSet::new();
        for card in deck.cards.iter() {
            hs_value_hash.insert(card.value_hash());
            hs_unique_hash.insert(card.unique_hash());
        }
        assert_eq!(hs_value_hash.len(), ALL_CARDS_NUM);
        assert_eq!(hs_unique_hash.len(), ALL_CARDS_NUM);

        // no card missing
        deck.shuffle();
        for card in deck.cards.iter() {
            assert!(hs_value_hash.contains(&card.value_hash()));
            assert!(hs_unique_hash.contains(&card.unique_hash()));
        }

        // shuffle from 1
        let mut diff = false;
        let card0 = deck.cards[0].unique_hash();
        for _ in 0..1000 {
            deck.shuffle_from(1);
            let card1 = deck.cards[0].unique_hash();
            if card0 != card1 {
                diff = true;
                break;
            }
        }
        assert!(!diff);

        // shuffle completely
        let mut diff = false;
        let card0 = deck.cards[0].unique_hash();
        for _ in 0..1000 {
            deck.shuffle();
            let card1 = deck.cards[0].unique_hash();
            if card0 != card1 {
                diff = true;
                break;
            }
        }
        assert!(diff);
    }
}
