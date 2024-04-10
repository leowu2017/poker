use std::cmp::Ordering;

use super::{hand::Hand, hand::Hands, rule::contain_natural};
use crate::cards::shoe::Shoe;
pub struct Game {
    pub shoe: Shoe,
    pub hands: Hands,
    pub results: Vec<Result>,
}
pub struct Result {
    hands: Hands,
    winner: Winner,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Winner {
    PLAYER,
    BANKER,
    TIE,
}

impl Game {
    pub fn new(deck_num: usize) -> Game {
        Game {
            shoe: Shoe::new(deck_num),
            hands: Hands {
                player: None,
                banker: None,
            },
            results: Vec::new(),
        }
    }

    fn draw_first_two(&mut self) {
        let [card0, card1, card2, card3] = self.shoe.draw_four();
        let player_hand = Hand::new(card0, card2);
        let bander_hand = Hand::new(card1, card3);
        self.hands.player = Some(player_hand);
        self.hands.banker = Some(bander_hand);
    }

    fn draw_third(&mut self) {
        if let (Some(player_hand), Some(banker_hand)) =
            (self.hands.player.as_mut(), self.hands.banker.as_mut())
        {
            if !contain_natural(player_hand, banker_hand) {
                let [card0, card1] = self.shoe.draw_two();
                player_hand.third = Some(card0);
                banker_hand.third = Some(card1);
            }
        }
    }

    fn get_winner(&self) -> Option<Winner> {
        if let (Some(player_hand), Some(banker_hand)) =
            (self.hands.player.as_ref(), self.hands.banker.as_ref())
        {
            Some(match player_hand.cmp(banker_hand) {
                Ordering::Greater => Winner::PLAYER,
                Ordering::Less => Winner::BANKER,
                Ordering::Equal => Winner::TIE,
            })
        } else {
            None
        }
    }

    pub fn play_one_round(&mut self) {
        self.draw_first_two();
        self.draw_third();
        if let Some(winner) = self.get_winner() {
            self.results.push(Result {
                hands: std::mem::replace(
                    &mut self.hands,
                    Hands {
                        player: None,
                        banker: None,
                    },
                ),
                winner: winner,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_test() {
        let deck_num = 8;
        let mut game = Game::new(deck_num);

        // A, A, 2 vs A, A, 2
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::TIE);
    }
}
