use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug)]

pub struct Calculator {
    pub state: State,
}

impl Calculator {
    pub fn new() -> Self {
        Self{
            state: State::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum NumericBase {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

const NUMERIC_BASE_KEYS: LazyLock<HashMap<NumericBase, Vec<char>>> = LazyLock::new(|| {
    HashMap::from([
        (NumericBase::Decimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']),
        (NumericBase::Hexadecimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']),
        (NumericBase::Octal, vec!['0', '1', '2', '3', '4', '5', '6', '7']),
        (NumericBase::Binary, vec!['0', '1']),
    ])
});

const NUMERIC_BASE_NAMES: LazyLock<HashMap<NumericBase, &str>> = LazyLock::new(|| {
    HashMap::from([
        (NumericBase::Decimal, "DEC"),
        (NumericBase::Hexadecimal, "HEX"),
        (NumericBase::Octal, "OCT"),
        (NumericBase::Binary, "BIN"),
    ])
});

#[derive(Debug)]
pub struct State {
    pub value: i128,
    pub numeric_base: NumericBase
}

impl State {
    fn new() -> Self {
        Self{
            value: 0,
            numeric_base: NumericBase::Decimal,
        }
    }
}
