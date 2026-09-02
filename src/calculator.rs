
#[derive(Debug, PartialEq)]
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
                //self.state.value = value;
                self.state.value.set(value);
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

    pub fn get_numeric_mode(&self) -> NumericMode {
        self.state.value.numeric_mode
    }
    pub fn set_numeric_mode(&mut self, numeric_mode: NumericMode) {
        self.state.value.numeric_mode = numeric_mode;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ValuePair {
    integer: i128,
    decimal: f64,
    numeric_mode: NumericMode,
}

impl ValuePair {
    fn set(&mut self, integer: i128) {
        self.integer = integer;
        self.decimal = integer as f64;
        self.numeric_mode = NumericMode::Integer;
    }
    pub fn get_integer(&self) -> i128 {
        self.integer
    }
    pub fn get_numeric_mode(&self) -> NumericMode {
        self.numeric_mode
    }
    pub fn get_decimal(&self) -> f64 {
        self.decimal
    }
}

use std::ops::AddAssign;
impl AddAssign<i128> for ValuePair {
    fn add_assign(&mut self, other: i128) {
        self.set(self.integer + other);
    }
}

use std::ops::SubAssign;
impl SubAssign<i128> for ValuePair {
    fn sub_assign(&mut self, other: i128) {
        self.set(self.integer - other);
    }
}

use std::ops::MulAssign;
impl MulAssign<i128> for ValuePair {
    fn mul_assign(&mut self, other: i128) {
        self.set(self.integer * other);
    }
}

use std::ops::DivAssign;
impl DivAssign<i128> for ValuePair {
    fn div_assign(&mut self, other: i128) {
        self.set(self.integer / other);
    }
}

use std::ops::RemAssign;
impl RemAssign<i128> for ValuePair {
    fn rem_assign(&mut self, other: i128) {
        self.set(self.integer % other);
    }
}

#[derive(Debug, Default)]
pub struct State {
    //value: i128,
    value: ValuePair,
    numeric_base: NumericBase,
    // TODO: Deprecate: maybe
    accumulator: Option<i128>,
    //accumulator: Option<ValuePair>,
    pending_operation: Option<Operation>,
    error: Option<Error>,
    //numeric_mode: NumericMode,
}

impl State {
    fn new() -> Self {
        let mut retval = Self::default();
        retval.clear();
        retval
    }

    pub fn get_value(&self) -> ValuePair {
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

/* kda_COMMENTED_OUT
    pub fn get_numeric_mode(&self) -> NumericMode {
        return self.numeric_mode;
    }
  kda_COMMENTED_OUT */

    pub fn get_error(&self) -> Option<Error> {
        return self.error;
    }

    pub fn clear(&mut self) {
        self.value.set(0);
        self.accumulator = None;
        self.pending_operation = None;
        self.error = None;
    }
}
