#[derive(PartialEq, Eq, Debug)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[allow(dead_code)]
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum OrderedSuit {
    Spade = 3,
    Heart = 2,
    Diamond = 1,
    Club = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suit_test() {
        assert_eq!(Suit::Spade, Suit::Spade);
        assert_ne!(Suit::Spade, Suit::Heart);
    }

    #[test]
    fn ordered_suit_test() {
        assert_eq!(OrderedSuit::Spade, OrderedSuit::Spade);
        assert_ne!(OrderedSuit::Spade, OrderedSuit::Heart);
        assert!(OrderedSuit::Spade != OrderedSuit::Heart);
        assert!(OrderedSuit::Spade > OrderedSuit::Heart);
        assert!(OrderedSuit::Heart > OrderedSuit::Diamond);
        assert!(OrderedSuit::Diamond > OrderedSuit::Club);
    }
}
