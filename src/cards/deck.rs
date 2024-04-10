use super::card::Card;

#[derive(Clone)]
pub struct Deck {
    pub cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Card::all_cards().clone(),
        }
    }
    pub fn shuffle(&mut self) {
        self.shuffle_from(0);
    }
    pub fn shuffle_from(&mut self, start_idx: usize) {
        use rand::Rng;
        let ref mut cards = self.cards;
        for idx in start_idx..cards.len() {
            let target_idx = rand::thread_rng().gen_range(idx..cards.len());
            cards.swap(idx, target_idx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_test() {
        use std::collections::HashSet;
        let mut hash = HashSet::new();
        let deck = Deck::new();
        for card in deck.cards.iter() {
            assert!(!hash.contains(card));
            hash.insert(card);
        }
    }
}
