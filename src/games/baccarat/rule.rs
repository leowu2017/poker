use super::card::Card;
use super::hand::Hand;
use crate::cards::traits::value::Value;

pub fn is_natural(hand: &Hand) -> bool {
    let value = hand.value();
    value == 8 || value == 9
}

pub fn contain_natural(hand1: &Hand, hand2: &Hand) -> bool {
    is_natural(hand1) || is_natural(hand2)
}

pub fn should_player_hit(hand: &Hand) -> bool {
    hand.value() <= 5
}

pub fn should_banker_hit(player_third: &Option<Card>, hand: &Hand) -> bool {
    let value = hand.value();
    match player_third {
        Some(ref card) => {
            let player_third = card.value();
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
    use crate::games::baccarat::number::Number;

    #[test]
    fn rule_card_value_test() {
        assert_eq!(Card::new(Suit::Spade, Number::Ace).value(), 1);
        assert_eq!(Card::new(Suit::Heart, Number::Four).value(), 4);
        assert_eq!(Card::new(Suit::Diamond, Number::Ten).value(), 0);
        assert_eq!(Card::new(Suit::Club, Number::King).value(), 0);
    }

    #[test]
    fn rule_natural_test() {
        // 4, 4
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Four),
            second: Card::new(Suit::Heart, Number::Four),
            third: None,
        };
        assert_eq!(is_natural(&hand), true);

        // 4. 5
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Four),
            second: Card::new(Suit::Spade, Number::Five),
            third: None,
        };
        assert_eq!(is_natural(&hand), true);

        // 4. 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Four),
            second: Card::new(Suit::Spade, Number::Three),
            third: None,
        };
        assert_eq!(is_natural(&hand), false);
    }

    #[test]
    fn rule_should_hit_test() {
        // 2, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Two),
            second: Card::new(Suit::Spade, Number::Three),
            third: None,
        };
        assert_eq!(should_player_hit(&hand), true);

        // 3, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Three),
            second: Card::new(Suit::Heart, Number::Three),
            third: None,
        };
        assert_eq!(should_player_hit(&hand), false);

        // _, _, 7 and 1, 2
        let player_third = Some(Card::new(Suit::Spade, Number::Seven));
        let hand = Hand {
            first: Card::new(Suit::Heart, Number::Ace),
            second: Card::new(Suit::Heart, Number::Two),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), true);

        // _, _, 8 and 1, 2
        let player_third = Some(Card::new(Suit::Spade, Number::Eight));
        let hand = Hand {
            first: Card::new(Suit::Heart, Number::Ace),
            second: Card::new(Suit::Heart, Number::Two),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), false);

        // _, _, 3 and 2, 3
        let player_third = Some(Card::new(Suit::Spade, Number::Three));
        let hand = Hand {
            first: Card::new(Suit::Heart, Number::Two),
            second: Card::new(Suit::Heart, Number::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), false);

        // _, _, 4 and 2, 3
        let player_third = Some(Card::new(Suit::Spade, Number::Four));
        let hand = Hand {
            first: Card::new(Suit::Heart, Number::Two),
            second: Card::new(Suit::Heart, Number::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&player_third, &hand), true);

        // _, _ and 3, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Three),
            second: Card::new(Suit::Heart, Number::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&None, &hand), false);

        // _, _ and 2, 3
        let hand = Hand {
            first: Card::new(Suit::Spade, Number::Two),
            second: Card::new(Suit::Spade, Number::Three),
            third: None,
        };
        assert_eq!(should_banker_hit(&None, &hand), true);
    }
}
