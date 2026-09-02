#[derive(Debug)]

#[derive(PartialEq)]
enum Mode {
    Accumulate,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    DivideByZero,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum NumericBase {
    #[default] Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum NumericMode {
    #[default]Integer,
    Decimal,
    Scientific,
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
                    Operation::Modulo => {
                        if value == 0 {
                            self.state.error = Some(Error::DivideByZero);
                            return;
                        }
                        self.state.value %= value
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

    pub fn set_numeric_mode(&mut self, numeric_mode: NumericMode) {
        self.state.numeric_mode = numeric_mode;
    }
}

#[derive(Debug, Default)]
pub struct State {
    value: i128,
    numeric_base: NumericBase,
    // TODO: Deprecate: maybe
    accumulator: Option<i128>,
    pending_operation: Option<Operation>,
    error: Option<Error>,
    numeric_mode: NumericMode,
}

impl State {
    fn new() -> Self {
        let mut retval = Self::default();
        retval.clear();
        retval
    }

    pub fn get_value(&self) -> i128 {
        return self.value;
    }

    pub fn get_numeric_base(&self) -> NumericBase {
        return self.numeric_base;
    }

    pub fn get_accumulator(&self) -> Option<i128> {
        return self.accumulator;
    }

    pub fn get_pending_operation(&self) -> Option<Operation> {
        return self.pending_operation;
    }

    pub fn get_numeric_mode(&self) -> NumericMode {
        return self.numeric_mode;
    }

    pub fn get_error(&self) -> Option<Error> {
        return self.error;
    }

    pub fn clear(&mut self) {
        self.value = 0;
        self.accumulator = None;
        self.pending_operation = None;
        self.error = None;
    }
}
