

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
    Float,
    Scientific,
}

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

    pub fn set_integer_accumulator(&mut self, value: i128) {
        if self.state.accumulator.is_some() {
            self.state.accumulator.as_mut().unwrap().set_integer(value);
        } else {
            let mut vp = ValuePair::default();
            vp.set_integer(value);
            self.state.accumulator = Some(vp);
        }
    }
    pub fn set_decimal_accumulator(&mut self, value: f64) {
        if self.state.accumulator.is_some() {
            self.state.accumulator.as_mut().unwrap().set_decimal(value);
        } else {
            let mut vp = ValuePair::default();
            vp.set_numeric_mode(self.state.value.get_numeric_mode());
            vp.set_decimal(value);
            self.state.accumulator = Some(vp);
        }
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
                        if value.get_integer() == 0 {
                            self.state.error = Some(Error::DivideByZero);
                            return;
                        }
                        self.state.value /= value
                    }
                    Operation::Modulo => {
                        if value.get_integer() == 0 {
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

    pub fn get_numeric_mode(&self) -> NumericMode {
        self.state.value.numeric_mode
    }
    pub fn set_numeric_mode(&mut self, numeric_mode: NumericMode) {
        self.state.value.set_numeric_mode(numeric_mode);
        if self.state.accumulator.is_some() {
            self.state.accumulator.as_mut().unwrap().set_numeric_mode(numeric_mode);
            assert_eq!(self.state.accumulator.unwrap().get_numeric_mode(), numeric_mode);
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ValuePair {
    integer: i128,
    decimal: f64,
    numeric_mode: NumericMode,
}

impl ValuePair {
    pub fn set_integer(&mut self, value: i128) {
        assert_eq!(self.numeric_mode, NumericMode::Integer);
        self.integer = value;
        self.decimal = value as f64;
    }
    pub fn set_decimal(&mut self, value: f64) {
        assert_ne!(self.numeric_mode, NumericMode::Integer);
        self.decimal = value;
        self.integer = value as i128;
    }
    fn clear(&mut self) {
        self.integer = 0;
        self.decimal = 0.0;
    }
    pub fn get_integer(&self) -> i128 {
        self.integer
    }
    pub fn get_decimal(&self) -> f64 {
        self.decimal
    }
    pub fn set_numeric_mode(&mut self, numeric_mode: NumericMode) {
        if self.numeric_mode == numeric_mode {
            return;
        }
        if self.numeric_mode == NumericMode::Integer {
            self.numeric_mode = numeric_mode;
            self.set_decimal(self.integer as f64);
        } else if numeric_mode == NumericMode::Integer {
            self.numeric_mode = numeric_mode;
            self.set_integer(self.decimal as i128);
        } else {
            self.numeric_mode = numeric_mode;
        }
    }
    pub fn get_numeric_mode(&self) -> NumericMode {
        self.numeric_mode
    }
}

use std::ops::AddAssign;
impl AddAssign<ValuePair> for ValuePair {
    fn add_assign(&mut self, other: ValuePair) {
        if self.numeric_mode == NumericMode::Integer {
            self.set_integer(self.integer + other.integer);
        } else {
            self.set_decimal(self.decimal + other.decimal);
        }
    }
}

use std::ops::SubAssign;
impl SubAssign<ValuePair> for ValuePair {
    fn sub_assign(&mut self, other: ValuePair) {
        if self.numeric_mode == NumericMode::Integer {
            self.set_integer(self.integer - other.integer);
        } else {
            self.set_decimal(self.decimal - other.decimal);
        }
    }
}

use std::ops::MulAssign;
impl MulAssign<ValuePair> for ValuePair {
    fn mul_assign(&mut self, other: ValuePair) {
        if self.numeric_mode == NumericMode::Integer {
            self.set_integer(self.integer * other.integer);
        } else {
            self.set_decimal(self.decimal * other.decimal);
        }
    }
}

use std::ops::DivAssign;
impl DivAssign<ValuePair> for ValuePair {
    fn div_assign(&mut self, other: ValuePair) {
        if self.numeric_mode == NumericMode::Integer {
            self.set_integer(self.integer / other.integer);
        } else {
            self.set_decimal(self.decimal / other.decimal);
        }
    }
}

use std::ops::RemAssign;
impl RemAssign<ValuePair> for ValuePair {
    fn rem_assign(&mut self, other: ValuePair) {
        if self.numeric_mode == NumericMode::Integer {
            self.set_integer(self.integer % other.integer);
        } else {
            self.set_decimal(self.decimal % other.decimal);
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    //value: i128,
    value: ValuePair,
    numeric_base: NumericBase,
    // TODO: Deprecate: maybe
    //accumulator: Option<i128>,
    accumulator: Option<ValuePair>,
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

    pub fn get_pending_operation(&self) -> Option<Operation> {
        return self.pending_operation;
    }

    pub fn get_error(&self) -> Option<Error> {
        return self.error;
    }

    pub fn clear(&mut self) {
        self.value.clear();
        self.accumulator = None;
        self.pending_operation = None;
        self.error = None;
    }
}
