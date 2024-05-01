use std::collections::HashMap;

use super::card::{all_cards, Card, ALL_CARDS_NUM};
use crate::cards::traits::deck::Deck;
use crate::cards::traits::hash::{UniqueHash, ValueHash, ValueHashT};
use crate::cards::traits::shoe::Shoe as ShoeTrait;

#[derive(Clone)]
pub struct Shoe {
    pub deck_num: usize,
    pub cards: Vec<Card>,
    card_idx: usize,                           // next draw position
    card_pos: HashMap<ValueHashT, Vec<usize>>, // card's value_hash -> vector of card position
}

impl Shoe {
    pub fn new(deck_num: usize) -> Shoe {
        let cards = all_cards();

        // repeatly copy deck
        let mut cards: Vec<Card> = cards
            .iter()
            .cloned()
            .cycle()
            .take(ALL_CARDS_NUM * deck_num)
            .collect();

        // update unique hash
        for card in cards.iter_mut() {
            card.update_unique_hash();
        }

        // calculate card_pos
        let mut card_pos = HashMap::new();
        for (idx, card) in cards.iter().enumerate() {
            let card_idxes = card_pos
                .entry(card.value_hash())
                .or_insert_with(|| Vec::with_capacity(deck_num));
            card_idxes.push(idx);
        }
        Shoe {
            deck_num: deck_num,
            cards: cards,
            card_idx: 0,
            card_pos: card_pos,
        }
    }

    pub fn shuffle(&mut self) {
        (self as &mut dyn Deck).shuffle();
        self.reset_card_idx();
    }
}

impl Deck for Shoe {
    fn card_num(&self) -> usize {
        self.cards.len()
    }
    fn swap(&mut self, idx1: usize, idx2: usize) {
        if (idx1 == idx2) || (self.cards[idx1].value_hash() == self.cards[idx2].value_hash()) {
            return;
        }
        self.move_idx(idx1, idx2);
        self.move_idx(idx2, idx1);
        self.cards.swap(idx1, idx2);
    }
}

impl ShoeTrait<Card> for Shoe {
    fn draw_n<const SIZE: usize>(&mut self) -> [Card; SIZE] {
        let cards = core::array::from_fn(|idx: usize| self.cards[self.card_idx + idx].clone());
        self.card_idx += SIZE;
        cards
    }

    fn get_card_idx(&self) -> usize {
        self.card_idx
    }

    fn reset_card_idx(&mut self) {
        self.card_idx = 0;
    }

    fn inc_card_idx(&mut self) {
        self.card_idx += 1;
    }

    fn get_card_ref(&self, idx: usize) -> &Card {
        &self.cards[idx]
    }

    fn get_card_map(&self, card: &Card) -> Option<&Vec<usize>> {
        self.card_pos.get(&card.value_hash())
    }

    fn get_mut_card_map_from_idx(&mut self, idx: usize) -> Option<&mut Vec<usize>> {
        let card = &self.cards[idx];
        self.card_pos.get_mut(&card.value_hash())
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::suit::Suit;
    use crate::cards::traits::hash::UniqueHash;
    use crate::games::baccarat::number::Number;

    use super::*;

    #[test]
    fn shoe_value_hash_test() {
        use std::collections::HashMap;
        const DECK_NUM: usize = 4;
        let shoe = Shoe::new(DECK_NUM);
        let mut map = HashMap::new();
        for card in shoe.cards.iter() {
            let value = map.entry(card.value_hash()).or_insert(0);
            *value += 1;
        }
        let all_cards = all_cards();
        for card in all_cards.iter() {
            assert_eq!(*map.get(&card.value_hash()).unwrap(), DECK_NUM);
        }
    }

    #[test]
    fn shoe_card_pos_test() {
        const DECK_NUM: usize = 4;
        let shoe = Shoe::new(DECK_NUM);

        // Spade Ace
        let card_pos = shoe
            .card_pos
            .get(&Card::new(Suit::Spade, Number::Ace).value_hash())
            .unwrap();
        assert_eq!(card_pos.len(), DECK_NUM);
        for i in 1..card_pos.len() {
            assert_eq!(card_pos[i] - card_pos[i - 1], 52);
        }

        // Heart 3
        let card_pos = shoe
            .card_pos
            .get(&Card::new(Suit::Heart, Number::Three).value_hash())
            .unwrap();
        assert_eq!(card_pos.len(), DECK_NUM);
        for i in 1..card_pos.len() {
            assert_eq!(card_pos[i] - card_pos[i - 1], 52);
        }
    }

    #[test]
    fn shoe_shuffle_test() {
        const DECK_NUM: usize = 4;
        let ref_shoe = Shoe::new(DECK_NUM);
        let mut shoe = ref_shoe.clone();

        // shuffle
        let mut diff = false;
        let card0 = shoe.cards[0].unique_hash();
        for _ in 0..1000 {
            shoe.shuffle();
            let card1 = shoe.cards[0].unique_hash();
            if card0 != card1 {
                diff = true;
                break;
            }
        }
        assert!(diff);

        // shuffle from middle
        let mut shoe = ref_shoe.clone();
        let shuffle_from = 5;
        shoe.shuffle_from(shuffle_from);
        for idx in 0..shuffle_from {
            assert_eq!(
                ref_shoe.cards[idx].unique_hash(),
                shoe.cards[idx].unique_hash()
            );
        }

        let mut diff = false;
        let card0 = shoe.cards[5].unique_hash();
        for _ in 0..1000 {
            shoe.shuffle_from(5);
            let card1 = shoe.cards[5].unique_hash();
            if card0 != card1 {
                diff = true;
                break;
            }
        }
        assert!(diff);

        // no shuffle
        let mut shoe = ref_shoe.clone();
        let shuffle_from = DECK_NUM * ALL_CARDS_NUM;
        shoe.shuffle_from(shuffle_from);
        for idx in 0..shuffle_from {
            assert_eq!(
                ref_shoe.cards[idx].unique_hash(),
                shoe.cards[idx].unique_hash()
            );
        }
    }

    #[test]
    fn shoe_draw_test() {
        const DECK_NUM: usize = 4;
        let mut shoe = Shoe::new(DECK_NUM);
        let ref_cards = shoe.cards.clone();

        let card = shoe.draw();
        assert_eq!(card.unique_hash(), ref_cards[0].unique_hash());
        assert_eq!(shoe.card_idx, 1);

        let cards = shoe.draw_two();
        assert_eq!(cards[0].unique_hash(), ref_cards[1].unique_hash());
        assert_eq!(cards[1].unique_hash(), ref_cards[2].unique_hash());
        assert_eq!(shoe.card_idx, 3);

        let cards = shoe.draw_three();
        assert_eq!(cards[0].unique_hash(), ref_cards[3].unique_hash());
        assert_eq!(cards[1].unique_hash(), ref_cards[4].unique_hash());
        assert_eq!(cards[2].unique_hash(), ref_cards[5].unique_hash());
        assert_eq!(shoe.card_idx, 6);
    }

    #[test]
    fn shoe_card_idx_test() {
        const DECK_NUM: usize = 4;
        let mut shoe = Shoe::new(DECK_NUM);
        assert_eq!(shoe.get_card_idx(), 0);

        shoe.draw();
        assert_eq!(shoe.get_card_idx(), 1);

        shoe.shuffle();
        assert_eq!(shoe.get_card_idx(), 0);

        shoe.draw();
        assert_eq!(shoe.get_card_idx(), 1);
    }

    #[test]
    fn shoe_find_card_idx_after_test() {
        const DECK_NUM: usize = 4;
        let shoe = Shoe::new(DECK_NUM);

        // first card after index 0
        let card = &shoe.cards[0];
        assert_eq!(shoe.find_card_idx_after(card, 0).unwrap(), 0);
        // first card after index 1
        assert_eq!(shoe.find_card_idx_after(card, 1).unwrap(), 52);
        // first card after index 52
        assert_eq!(shoe.find_card_idx_after(card, 52).unwrap(), 52);
        // first card after index 53
        assert_eq!(shoe.find_card_idx_after(card, 53).unwrap(), 52 * 2);
    }

    #[test]
    fn shoe_swap_test() {
        const DECK_NUM: usize = 4;
        let mut shoe = Shoe::new(DECK_NUM);
        // 0, 1
        let card0 = shoe.cards[0].clone();
        let card1 = shoe.cards[1].clone();
        shoe.swap(0, 1);
        assert_eq!(shoe.cards[0].unique_hash(), card1.unique_hash());
        assert_eq!(shoe.cards[1].unique_hash(), card0.unique_hash());
        assert_eq!(shoe.card_pos.get(&card0.value_hash()).unwrap()[0], 1);
        assert_eq!(shoe.card_pos.get(&card1.value_hash()).unwrap()[0], 0);
        // 1, 0
        shoe.swap(1, 0);
        assert_eq!(shoe.cards[0].unique_hash(), card0.unique_hash());
        assert_eq!(shoe.cards[1].unique_hash(), card1.unique_hash());
        assert_eq!(shoe.card_pos.get(&card0.value_hash()).unwrap()[0], 0);
        assert_eq!(shoe.card_pos.get(&card1.value_hash()).unwrap()[0], 1);
        // 0, 52
        let card0 = shoe.cards[0].clone();
        let card52 = shoe.cards[52].clone();
        shoe.swap(0, 52);
        assert_eq!(card0.value_hash(), card52.value_hash());
        assert_eq!(shoe.cards[0].value_hash(), card52.value_hash());
        assert_eq!(shoe.cards[52].value_hash(), card0.value_hash());
        assert_eq!(shoe.card_pos.get(&card0.value_hash()).unwrap()[0], 0);
        assert_eq!(shoe.card_pos.get(&card0.value_hash()).unwrap()[1], 52);
        assert_eq!(shoe.card_pos.get(&card52.value_hash()).unwrap()[0], 0);
        assert_eq!(shoe.card_pos.get(&card52.value_hash()).unwrap()[1], 52);
    }

    #[test]
    fn shoe_set_card_test() {
        const DECK_NUM: usize = 4;
        let mut shoe = Shoe::new(DECK_NUM);

        let mut inserted_cards =Vec::new();
        // get first card
        let card0 = shoe.cards.last().unwrap().clone();

        // insert card0 DECK_NUM times
        for _ in 0..DECK_NUM {
            inserted_cards.push(&card0);
            let result = shoe.set_card(&card0);
            assert_eq!(result, Ok(()));
        }

        // next card
        let card1 = shoe.cards.last().unwrap().clone();
        assert_ne!(card0.value_hash(), card1.value_hash());

        // insert card1 DECK_NUM times
        for _ in 0..DECK_NUM {
            inserted_cards.push(&card1);
            let result = shoe.set_card(&card1);
            assert_eq!(result, Ok(()));
        }

        // check first DECK_NUM * 2 values
        for idx in 0..(DECK_NUM * 2) {
            assert_eq!(shoe.cards[idx].value_hash(), inserted_cards[idx].value_hash());
        }
    }
}
