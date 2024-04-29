use crate::cards::traits::value::Value;

use super::card::Card;

#[derive(Clone)]
pub struct Hand {
    pub first: Card,
    pub second: Card,
    pub third: Option<Card>,
}

impl Hand {
    pub fn new(first: Card, second: Card) -> Hand {
        Hand {
            first: first,
            second: second,
            third: None,
        }
    }
}

impl Value for Hand {
    fn value(&self) -> i32 {
        let first = self.first.value();
        let second = self.second.value();
        let third = match self.third {
            Some(ref card) => card.value(),
            None => 0,
        };
        (first + second + third) % 10
    }
}
pub struct Hands {
    pub player: Option<Hand>,
    pub banker: Option<Hand>,
}

impl Hands {
    pub fn new(player_hand: Option<Hand>, banker_hand: Option<Hand>) -> Hands {
        Hands {
            player: player_hand,
            banker: banker_hand,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::suit::Suit;
    use crate::games::baccarat::number::Number;

    #[test]
    fn hand_value_test() {
        // A, 2
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Ace),
            second: Card::new(Suit::Spade, Number::Two),
            third: None,
        };
        assert_eq!(hand.value(), 3);

        // A, 9
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Ace),
            second: Card::new(Suit::Spade, Number::Nine),
            third: None,
        };
        assert_eq!(hand.value(), 0);

        // J, Q
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Jack),
            second: Card::new(Suit::Spade, Number::Queen),
            third: None,
        };
        assert_eq!(hand.value(), 0);

        // 7, 8, 9
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Seven),
            second: Card::new(Suit::Spade, Number::Eight),
            third: Some(Card::new(Suit::Spade, Number::Nine)),
        };
        assert_eq!(hand.value(), 4);
    }

    #[test]
    fn hand_value_compare_test() {
        // A, 2 vs 2, J
        let hand1 = Hand {
            first: Card::new(Suit::Spade, Number::Ace),
            second: Card::new(Suit::Spade, Number::Two),
            third: None,
        };
        let hand2 = Hand {
            first: Card::new(Suit::Heart, Number::Two),
            second: Card::new(Suit::Heart, Number::Jack),
            third: None,
        };
        assert!(hand1.value() > hand2.value());

        // 5, 4 vs 2, J, 7
        let hand1 = Hand {
            first: Card::new(Suit::Spade, Number::Five),
            second: Card::new(Suit::Spade, Number::Four),
            third: None,
        };
        let hand2 = Hand {
            first: Card::new(Suit::Heart, Number::Two),
            second: Card::new(Suit::Heart, Number::Jack),
            third: Some(Card::new(Suit::Heart, Number::Seven)),
        };
        assert_eq!(hand1.value(), hand2.value());

        // 5, 4, 3 vs 2, J, 7
        let hand1 = Hand {
            first: Card::new(Suit::Spade, Number::Five),
            second: Card::new(Suit::Spade, Number::Four),
            third: Some(Card::new(Suit::Spade, Number::Three)),
        };
        let hand2 = Hand {
            first: Card::new(Suit::Heart, Number::Two),
            second: Card::new(Suit::Heart, Number::Jack),
            third: Some(Card::new(Suit::Heart, Number::Seven)),
        };
        assert!(hand1.value() < hand2.value());
    }
}
