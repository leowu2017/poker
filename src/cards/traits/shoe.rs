use super::deck::Deck;
use super::hash::ValueHash;
use std::usize;

pub trait Shoe<Card: Sized + ValueHash>: Deck {
    //
    // Draw cards
    //

    fn draw_n<const SIZE: usize>(&mut self) -> [Card; SIZE];

    fn draw(&mut self) -> Card {
        let [card] = self.draw_n::<1>();
        card
    }

    fn draw_two(&mut self) -> [Card; 2] {
        self.draw_n::<2>()
    }

    fn draw_three(&mut self) -> [Card; 3] {
        self.draw_n::<3>()
    }

    fn draw_four(&mut self) -> [Card; 4] {
        self.draw_n::<4>()
    }

    //
    // Manipulate card
    //
    fn get_card_idx(&self) -> usize;
    fn reset_card_idx(&mut self);
    fn inc_card_idx(&mut self);

    fn get_card_map(&self, card: &Card) -> Option<&Vec<usize>>;
    fn get_mut_card_map_from_idx(&mut self, idx: usize) -> Option<&mut Vec<usize>>;

    fn get_card_ref(&self, idx: usize) -> &Card;

    // inclusive
    fn find_card_idx_after(&self, card: &Card, after: usize) -> Option<usize> {
        let card_map = self.get_card_map(card);
        match card_map {
            Some(card_map) => Some(match card_map.binary_search(&after) {
                Ok(idx) => card_map[idx],
                Err(idx) => card_map[idx],
            }),
            None => None,
        }
    }

    fn move_idx(&mut self, idx_from: usize, idx_to: usize) {
        if idx_from == idx_to {
            return;
        }
        let card_map = match self.get_mut_card_map_from_idx(idx_from) {
            None => {
                return;
            }
            Some(card_map) => card_map,
        };
        let map_idx_from = match card_map.binary_search(&idx_from) {
            Ok(idx) => idx,
            Err(_) => {
                panic!("Should find card.");
            }
        };
        let map_idx_to = match card_map.binary_search(&idx_to) {
            Ok(_) => {
                // idx_from and idx_to are identical cards.
                return;
            }
            Err(idx) => idx,
        };
        if map_idx_from < map_idx_to {
            for map_idx in map_idx_from..(map_idx_to - 1) {
                card_map[map_idx] = card_map[map_idx + 1];
            }
            card_map[map_idx_to - 1] = idx_to;
        } else if map_idx_from > map_idx_to {
            for map_idx in (map_idx_to..map_idx_from).rev() {
                card_map[map_idx + 1] = card_map[map_idx];
            }
            card_map[map_idx_to] = idx_to;
        } else {
            // map_idx_from == map_idx_to
            card_map[map_idx_from] = idx_to;
        }
    }

    fn set_card(&mut self, target_card: &Card) -> Result<(), &'static str> {
        let card_idx = self.get_card_idx();
        if target_card.value_hash() == self.get_card_ref(card_idx).value_hash() {
            self.inc_card_idx();
            return Ok(());
        }
        let target_idx = self.find_card_idx_after(target_card, card_idx);
        let Some(target_idx) = target_idx else {
            return Err("card does not exist");
        };
        self.swap(card_idx, target_idx);
        self.inc_card_idx();
        Ok(())
    }
}
