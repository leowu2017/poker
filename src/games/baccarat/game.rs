use super::rule::{contain_natural, should_banker_hit, should_player_hit};
use super::{hand::Hand, hand::Hands, shoe::Shoe};
use crate::cards::traits::shoe::Shoe as ShoeTrait;
use crate::cards::traits::value::Value;

pub struct Game {
    pub shoe: Shoe,
    pub hands: Hands,
    pub results: Vec<Result>,
}
pub struct Result {
    pub hands: Hands,
    pub winner: Winner,
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
                if should_player_hit(player_hand) {
                    let card = self.shoe.draw();
                    player_hand.third = Some(card);
                }
                if should_banker_hit(&player_hand.third, banker_hand) {
                    let card = self.shoe.draw();
                    banker_hand.third = Some(card);
                }
                // let [card0, card1] = self.shoe.draw_two();
                // player_hand.third = Some(card0);
                // banker_hand.third = Some(card1);
            }
        }
    }

    fn get_winner(&self) -> Option<Winner> {
        if let (Some(player_hand), Some(banker_hand)) =
            (self.hands.player.as_ref(), self.hands.banker.as_ref())
        {
            Some(if player_hand.value() > banker_hand.value() {
                Winner::PLAYER
            } else if player_hand.value() < banker_hand.value() {
                Winner::BANKER
            } else {
                Winner::TIE
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

        // A, 3, 5 vs 2, 4
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::PLAYER);

        // 6, 8, T vs 7, 9
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::BANKER);

        // J, K, 2 vs Q, A, 3
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::BANKER);

        // 4, 6, 8 vs 5, 7, 9
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::PLAYER);

        // T, Q, A vs J, K, 2
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::BANKER);

        // 3, 5 vs 4, 6
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::PLAYER);

        // 7, 9 vs 8, T
        game.play_one_round();
        assert!(!game.results.last().is_none());
        let result = game.results.last().unwrap();
        assert_eq!(result.winner, Winner::BANKER);
    }
}
