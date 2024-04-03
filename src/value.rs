#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_test() {
        assert_eq!(Value::Two, Value::Two);
        assert_ne!(Value::Two, Value::Three);
        assert!(Value::Two != Value::Three);
        assert!(Value::Two < Value::Three);
        assert!(Value::Three < Value::Four);
        assert!(Value::Four < Value::Five);
        assert!(Value::Five < Value::Six);
        assert!(Value::Six < Value::Seven);
        assert!(Value::Seven < Value::Eight);
        assert!(Value::Eight < Value::Nine);
        assert!(Value::Nine < Value::Ten);
        assert!(Value::Ten < Value::Jack);
        assert!(Value::Jack < Value::Queen);
        assert!(Value::Queen < Value::King);
        assert!(Value::King < Value::Ace);
    }
}
