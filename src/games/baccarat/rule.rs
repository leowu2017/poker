use super::hand::{hand_value, Hand};
use crate::cards::card::Card;
use crate::cards::value::Value;

pub fn card_value(card: &Card) -> u8 {
    use std::collections::HashMap;
    lazy_static! {
        static ref MAP: HashMap<Value, u8> = {
            use Value::*;
            let mut map = HashMap::new();
            map.insert(Ace, 1);
            map.insert(Two, 2);
            map.insert(Three, 3);
            map.insert(Four, 4);
            map.insert(Five, 5);
            map.insert(Six, 6);
            map.insert(Seven, 7);
            map.insert(Eight, 8);
            map.insert(Nine, 9);
            map.insert(Ten, 0);
            map.insert(Jack, 0);
            map.insert(Queen, 0);
            map.insert(King, 0);
            map
        };
    }
    MAP[&card.value]
}

pub fn is_natural(hand: &Hand) -> bool {
    let value = hand_value(hand);
    value == 8 || value == 9
}

pub fn contain_natural(hand1: &Hand, hand2: &Hand) -> bool {
    is_natural(hand1) || is_natural(hand2)
}

pub fn should_player_hit(hand: &Hand) -> bool {
    let value = hand_value(hand);
    value <= 5
}

pub fn should_banker_hit(player_third: &Option<Card>, hand: &Hand) -> bool {
    let value = hand_value(hand);
    match player_third {
        Some(ref card) => {
            let player_third = card_value(card);
            if value <= 2 {
                true
            } else if value == 3 {
                player_third != 8
            } else if value == 4 {
                player_third >= 2 && player_third <= 7
            } else if value == 5 {
                player_third >= 4 && player_third <= 7
            } else if value == 6 {
                player_third == 6 || player_third == 7
            } else {
                false
            }
        }
        None => value <= 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::suit::Suit;

    #[test]
    fn rule_card_value_test() {
        assert_eq!(card_value(&Card::new(Suit::Spade, Value::Ace)), 1);
        assert_eq!(card_value(&Card::new(Suit::Heart, Value::Four)), 4);
        assert_eq!(card_value(&Card::new(Suit::Diamond, Value::Ten)), 0);
        assert_eq!(card_value(&Card::new(Suit::Club, Value::King)), 0);
    }

    #[test]
    fn rule_natural_test() {
        // 4, 4
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Four),
            second: Card::new(Suit::Heart, Value::Four),
            third: None,
        };
        assert_eq!(is_natural(&hand), true);

        // 4. 5
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Four),
            second: Card::new(Suit::Spade, Value::Five),
            third: None,
        };
        assert_eq!(is_natural(&hand), true);

        // 4. 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Four),
            second: Card::new(Suit::Spade, Value::Three),
            third: None,
        };
        assert_eq!(is_natural(&hand), false);
    }

    #[test]
    fn rule_should_hit_test() {
        // 2, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Two),
            second: Card::new(Suit::Spade, Value::Three),
            third: None,
        };
        assert_eq!(should_player_hit(&hand), true);

        // 3, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Three),
            second: Card::new(Suit::Heart, Value::Three),
            third: None,
        };
        assert_eq!(should_player_hit(&hand), false);

        // _, _, 7 and 1, 2
        let player_third = Some(Card::new(Suit::Spade, Value::Seven));
        let hand = Hand {
            first: Card::new(Suit::Heart, Value::Ace),
            second: Card::new(Suit::Heart, Value::Two),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), true);

        // _, _, 8 and 1, 2
        let player_third = Some(Card::new(Suit::Spade, Value::Eight));
        let hand = Hand {
            first: Card::new(Suit::Heart, Value::Ace),
            second: Card::new(Suit::Heart, Value::Two),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), false);

        // _, _, 3 and 2, 3
        let player_third = Some(Card::new(Suit::Spade, Value::Three));
        let hand = Hand {
            first: Card::new(Suit::Heart, Value::Two),
            second: Card::new(Suit::Heart, Value::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), false);

        // _, _, 4 and 2, 3
        let player_third = Some(Card::new(Suit::Spade, Value::Four));
        let hand = Hand {
            first: Card::new(Suit::Heart, Value::Two),
            second: Card::new(Suit::Heart, Value::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), true);

        // _, _ and 3, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Three),
            second: Card::new(Suit::Heart, Value::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&None, &hand), false);

        // _, _ and 2, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Value::Two),
            second: Card::new(Suit::Spade, Value::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&None, &hand), true);
    }
}
