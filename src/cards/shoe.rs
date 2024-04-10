use super::card::Card;
#[derive(Clone)]
pub struct Shoe {
    pub cards: Vec<Card>,
    pub card_idx: usize,
}

impl Shoe {
    pub fn new(deck_num: usize) -> Shoe {
        let cards = Card::all_cards();
        let cards: Vec<Card> = cards
            .iter()
            .cloned()
            .cycle()
            .take(cards.len() * deck_num)
            .collect();
        Shoe {
            cards: cards,
            card_idx: 0,
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
    pub fn redraw(&mut self) {
        self.card_idx = 0;
    }

    #[inline]
    pub fn draw_n<const SIZE: usize>(&mut self) -> [Card; SIZE] {
        let cards = core::array::from_fn(|idx: usize| self.cards[self.card_idx + idx].clone());
        self.card_idx += SIZE;
        cards
    }

    pub fn draw(&mut self) -> Card {
        let [card] = self.draw_n::<1>();
        card
    }

    pub fn draw_two(&mut self) -> [Card; 2] {
        self.draw_n::<2>()
    }

    pub fn draw_three(&mut self) -> [Card; 3] {
        self.draw_n::<3>()
    }

    pub fn draw_four(&mut self) -> [Card; 4] {
        self.draw_n::<4>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoe_unique_cards_test() {
        use std::collections::HashMap;
        let deck_num = 8;
        let shoe = Shoe::new(deck_num);
        let mut map: HashMap<&Card, usize> = HashMap::new();
        for card in shoe.cards.iter() {
            let value = map.entry(card).or_insert(0);
            *value += 1;
        }
        let all_cards = Card::all_cards();
        for card in all_cards.iter() {
            assert_eq!(*map.get(card).unwrap(), deck_num);
        }
    }

    #[test]
    fn shoe_shuffle_test() {
        let deck_num = 8;
        let ref_shoe = Shoe::new(deck_num);

        // shuffle from middle
        let mut shoe = ref_shoe.clone();
        let shuffle_from = 5;
        shoe.shuffle_from(shuffle_from);
        for idx in 0..shuffle_from {
            assert_eq!(ref_shoe.cards[idx], shoe.cards[idx]);
        }

        // no shuffle
        let mut shoe = ref_shoe.clone();
        let shuffle_from = deck_num * 52;
        shoe.shuffle_from(shuffle_from);
        for idx in 0..shuffle_from {
            assert_eq!(ref_shoe.cards[idx], shoe.cards[idx]);
        }
    }

    #[test]
    fn shoe_draw_test() {
        let ref_cards = Card::all_cards();
        let deck_num = 8;
        let mut shoe = Shoe::new(deck_num);
        let card = shoe.draw();
        assert_eq!(card, ref_cards[0]);
        assert_eq!(shoe.card_idx, 1);
        let cards = shoe.draw_two();
        assert_eq!(cards[0], ref_cards[1]);
        assert_eq!(cards[1], ref_cards[2]);
        assert_eq!(shoe.card_idx, 3);
        let cards = shoe.draw_three();
        assert_eq!(cards[0], ref_cards[3]);
        assert_eq!(cards[1], ref_cards[4]);
        assert_eq!(cards[2], ref_cards[5]);
        assert_eq!(shoe.card_idx, 6);
    }
}
