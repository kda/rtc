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
    _mode: Mode,
    pub state: State,
}

impl Calculator {
    pub fn new() -> Self {
        Self{
            _mode: Mode::Accumulate,
            state: State::new(),
        }
    }

    pub fn set_accumulator(&mut self, value: i128) {
        self.state.accumulator = Some(value);
    }

    pub fn clear_accumulator(&mut self) {
        self.state.accumulator = None;
    }

    pub fn set_pending_operation(&mut self, operation: Operation) {
        self.state.pending_operation = Some(operation);
    }

    pub fn update_value(&mut self) {
        if let Some(value) = self.state.accumulator {
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
            self.state.accumulator = None;
            self.state.pending_operation = None;
        }
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }

    pub fn set_numeric_base(&mut self, numeric_base: NumericBase) {
        self.state.numeric_base = numeric_base;
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
    pub accumulator: Option<i128>,
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
        self.accumulator = None;
        self.pending_operation = None;
        self.error = None;
    }
}
