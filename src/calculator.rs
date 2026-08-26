use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug)]

#[derive(PartialEq)]
enum Mode {
    Accumulate,
}

#[derive(Debug)]
pub struct Calculator {
    mode: Mode,
    pub state: State,
}

impl Calculator {
    pub fn new() -> Self {
        Self{
            mode: Mode::Accumulate,
            state: State::new(),
        }
    }

    pub fn accumulate(&mut self, c: char) {
        if self.mode == Mode::Accumulate {
            self.state.accumulator.push(c);
        }
    }

    pub fn update_value(&mut self) {
        if self.state.accumulator.len() > 0 {
            self.state.value = self.state.accumulator.parse::<i128>().unwrap();
            self.state.accumulator.clear();
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NumericBase {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

pub const NUMERIC_BASE_KEYS: LazyLock<HashMap<NumericBase, Vec<char>>> = LazyLock::new(|| {
    HashMap::from([
        (NumericBase::Decimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']),
        (NumericBase::Hexadecimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']),
        (NumericBase::Octal, vec!['0', '1', '2', '3', '4', '5', '6', '7']),
        (NumericBase::Binary, vec!['0', '1']),
    ])
});

pub const NUMERIC_BASE_NAMES: LazyLock<HashMap<NumericBase, &str>> = LazyLock::new(|| {
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
    pub numeric_base: NumericBase,
    pub accumulator: String,
}

impl State {
    fn new() -> Self {
        Self{
            value: 0,
            numeric_base: NumericBase::Decimal,
            accumulator: String::new(),
        }
    }
}
