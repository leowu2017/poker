use super::rule::card_value;
use crate::cards::card::Card;

#[derive(Eq, Clone, Debug)]
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

pub fn hand_value(hand: &Hand) -> u8 {
    let first = card_value(&hand.first);
    let second = card_value(&hand.second);
    let third = match hand.third {
        Some(ref card) => card_value(card),
        None => 0,
    };
    (first + second + third) % 10
}

use std::cmp::PartialEq;
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        hand_value(&self).eq(&hand_value(&other))
    }
}

use std::cmp::Ordering;
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        hand_value(&self).cmp(&hand_value(&other))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Hands {
    pub player: Option<Hand>,
    pub banker: Option<Hand>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::suit::Suit;
    use crate::cards::value::Value;

    #[test]
    fn hand_value_test() {
        // A, 2
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Ace),
            second: Card::new(Suit::Spade, Value::Two),
            third: None,
        };
        assert_eq!(hand_value(&hand), 3);

        // A, 9
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Ace),
            second: Card::new(Suit::Spade, Value::Nine),
            third: None,
        };
        assert_eq!(hand_value(&hand), 0);

        // J, Q
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Jack),
            second: Card::new(Suit::Spade, Value::Queen),
            third: None,
        };
        assert_eq!(hand_value(&hand), 0);

        // 7, 8, 9
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Seven),
            second: Card::new(Suit::Spade, Value::Eight),
            third: Some(Card::new(Suit::Spade, Value::Nine)),
        };
        assert_eq!(hand_value(&hand), 4);
    }

    #[test]
    fn hand_order_test() {
        // A, 2 vs 2, J
        let hand1 = Hand {
            first: Card::new(Suit::Spade, Value::Ace),
            second: Card::new(Suit::Spade, Value::Two),
            third: None,
        };
        let hand2 = Hand {
            first: Card::new(Suit::Heart, Value::Two),
            second: Card::new(Suit::Heart, Value::Jack),
            third: None,
        };
        assert!(hand1 > hand2);

        // 5, 4 vs 2, J, 7
        let hand1 = Hand {
            first: Card::new(Suit::Spade, Value::Five),
            second: Card::new(Suit::Spade, Value::Four),
            third: None,
        };
        let hand2 = Hand {
            first: Card::new(Suit::Heart, Value::Two),
            second: Card::new(Suit::Heart, Value::Jack),
            third: Some(Card::new(Suit::Heart, Value::Seven)),
        };
        assert!(hand1 == hand2);

        // 5, 4, 3 vs 2, J, 7
        let hand1 = Hand {
            first: Card::new(Suit::Spade, Value::Five),
            second: Card::new(Suit::Spade, Value::Four),
            third: Some(Card::new(Suit::Spade, Value::Three)),
        };
        let hand2 = Hand {
            first: Card::new(Suit::Heart, Value::Two),
            second: Card::new(Suit::Heart, Value::Jack),
            third: Some(Card::new(Suit::Heart, Value::Seven)),
        };
        assert!(hand1 < hand2);
    }
}
