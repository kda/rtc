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

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Error {
    DivideByZero,
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

    pub fn unaccumulate(&mut self) {
        if ! self.state.accumulator.is_empty() {
            self.state.accumulator.pop();
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
                    Operation::Divide => {
                        if value == 0 {
                            self.state.error = Some(Error::DivideByZero);
                            return;
                        }
                        self.state.value /= value
                    }
                }
            } else {
                self.state.value = value;
            }

            // clean up
            self.state.accumulator.clear();
            self.state.pending_operation = None;
        }
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub enum NumericBase {
    #[default] Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

#[derive(Debug, Default)]
pub struct State {
    pub value: i128,
    pub numeric_base: NumericBase,
    pub accumulator: String,
    pub pending_operation: Option<Operation>,
    pub error: Option<Error>,
}

impl State {
    fn new() -> Self {
        let mut retval = Self::default();
        // A little ugly, but ensures default state matches
        retval.clear();
        retval
    }

    pub fn clear(&mut self) {
        self.value = 0;
        self.accumulator.clear();
        self.pending_operation = None;
        self.error = None;
    }
}
