#[derive(Debug)]

#[derive(PartialEq)]
enum Mode {
    Accumulate,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
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

    pub fn set_pending_operation(&mut self, operation: Operation) {
        self.state.pending_operation = Some(operation);
    }

    pub fn update_value(&mut self) {
        if self.state.accumulator.len() > 0 {
            let value = self.state.accumulator.parse::<i128>().unwrap();
            if let Some(operation) = &self.state.pending_operation {
                match operation {
                    Operation::Add => self.state.value += value,
                    Operation::Subtract => self.state.value -= value,
                    Operation::Multiply => self.state.value *= value,
                    Operation::Divide => self.state.value /= value,
                }
            } else {
                self.state.value = value;
            }

            // clean up
            self.state.accumulator.clear();
            self.state.pending_operation = None;
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

#[derive(Debug)]
pub struct State {
    pub value: i128,
    pub numeric_base: NumericBase,
    pub accumulator: String,
    pub pending_operation: Option<Operation>,
}

impl State {
    fn new() -> Self {
        Self{
            value: 0,
            numeric_base: NumericBase::Decimal,
            accumulator: String::new(),
            pending_operation: None,
        }
    }
}
