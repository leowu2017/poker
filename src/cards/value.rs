#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash, Debug)]
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

impl Value {
    pub fn all_values() -> &'static [Value; 13] {
        use Value::*;
        static ALL_VALUES: [Value; 13] = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];
        &ALL_VALUES
    }
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
