use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::{Line, Position, Size, Text},
};
use ratatui::layout::Rect;
//use ratatui::symbols::merge::MergeStrategy;
use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use std::collections::HashMap;
use std::sync::LazyLock;
//use std::ops::Deref;

mod calculator;

use calculator::Calculator;
use calculator::NumericBase;
use calculator::NumericMode;
use calculator::Operation;
use calculator::Error;


pub const NUMERIC_BASE_ENTRY_KEYS: LazyLock<HashMap<NumericBase, Vec<char>>> = LazyLock::new(|| {
    HashMap::from([
        (NumericBase::Decimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']),
        (NumericBase::Hexadecimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']),
        (NumericBase::Octal, vec!['0', '1', '2', '3', '4', '5', '6', '7']),
        (NumericBase::Binary, vec!['0', '1']),
    ])
});

pub const NUMERIC_BASE_KEYS: LazyLock<HashMap<char, NumericBase>> = LazyLock::new(|| {
    HashMap::from([
        ('D', NumericBase::Decimal),
        ('H', NumericBase::Hexadecimal),
        ('O', NumericBase::Octal),
        ('B', NumericBase::Binary),
    ])
});

pub const NUMERIC_BASE_HELP: LazyLock<HashMap<NumericBase, &str>> = LazyLock::new(|| {
    HashMap::from([
        (NumericBase::Decimal, "Dec"),
        (NumericBase::Hexadecimal, "Hex"),
        (NumericBase::Octal, "Oct"),
        (NumericBase::Binary, "Bin"),
    ])
});

pub const NUMERIC_BASE_NAMES: LazyLock<HashMap<NumericBase, &str>> = LazyLock::new(|| {
    HashMap::from([
        (NumericBase::Decimal, "DEC (10)"),
        (NumericBase::Hexadecimal, "HEX (16)"),
        (NumericBase::Octal, "OCT (8)"),
        (NumericBase::Binary, "BIN (2)"),
    ])
});

pub const NUMERIC_MODE_KEYS: LazyLock<HashMap<char, NumericMode>> = LazyLock::new(|| {
    HashMap::from([
        ('I', NumericMode::Integer),
        ('A', NumericMode::Float),
        ('S', NumericMode::Scientific),
    ])
});

pub const NUMERIC_MODE_NAMES: LazyLock<HashMap<NumericMode, &str>> = LazyLock::new(|| {
    HashMap::from([
        (NumericMode::Integer, "int"),
        (NumericMode::Float, "dec"),
        (NumericMode::Scientific, "sci"),
    ])
});

pub const OPERATION_KEYS: LazyLock<HashMap<char, Operation>> = LazyLock::new(|| {
    HashMap::from([
        ('+', Operation::Add),
        ('-', Operation::Subtract),
        ('*', Operation::Multiply),
        ('/', Operation::Divide),
        ('%', Operation::Modulo),
    ])
});

pub const OPERATION_NAMES: LazyLock<HashMap<Operation, &str>> = LazyLock::new(|| {
    HashMap::from([
        (Operation::Add, "+"),
        (Operation::Subtract, "-"),
        (Operation::Multiply, "*"),
        (Operation::Divide, "/"),
        (Operation::Modulo, "%"),
    ])
});

pub const ERROR_NAMES: LazyLock<HashMap<Error, &str>> = LazyLock::new(|| {
    HashMap::from([
        (Error::DivideByZero, "divide by zero"),
    ])
});


#[derive(Debug)]
pub struct App {
    exit_requested: bool,
    size: Size,
    calculator: Calculator,
    accumulator: String,

    // Display oriented values
    significant_digits: usize,
}

impl App {
    fn new() -> Self {
        Self {
            exit_requested: false,
            size: Size::default(),
            calculator: Calculator::new(),
            accumulator: String::new(),
            significant_digits: 2,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.size = terminal.size()?;

        while !self.exit_requested {
            terminal.draw(|frame| self.draw(frame))?;
						self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        frame.set_cursor_position(Position{x: self.size.width - 1, y: self.size.height - 1});
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Backspace => {
                if self.calculator.state.get_error().is_none() {
                    if self.accumulator.len() > 0 {
                        self.accumulator.pop();
                    }
                } else {
                    self.calculator.clear();
                    self.accumulator.clear();
                }
                self.parse_and_update_accumulator();
            }
            KeyCode::Esc => self.calculator.clear(),
            KeyCode::Char('q') => self.request_exit(),
            KeyCode::Enter | KeyCode::Char('=') => {
                self.calculator.update_value();
                self.accumulator.clear();
                self.parse_and_update_accumulator();
            },
            KeyCode::Char('.') => {
                if self.calculator.get_numeric_mode() != NumericMode::Integer && self.accumulator.find('.').is_none() {
                    self.accumulator.push('.');
                    self.parse_and_update_accumulator();
                }
            }
            KeyCode::Char('E') => {
            }
            KeyCode::Char(c) => {
                if NUMERIC_BASE_ENTRY_KEYS[&self.calculator.state.get_numeric_base()].contains(&c) {
                    self.accumulator.push(c);
                } else if let Some(operation) = OPERATION_KEYS.get(&c) {
                    self.calculator.update_value();
                    self.calculator.set_pending_operation(operation.clone());
                    self.accumulator.clear();
                } else if let Some(base) = NUMERIC_BASE_KEYS.get(&c) {
                    self.calculator.set_numeric_base(*base);
                } else if let Some(mode) = NUMERIC_MODE_KEYS.get(&c) {
                    self.calculator.set_numeric_mode(*mode);
                }
                self.parse_and_update_accumulator();
            }
            _ => {
            }
        }
    }

    fn request_exit(&mut self) {
        self.exit_requested = true;
    }

    fn format_value_pair(&self, value: calculator::ValuePair) -> String {
        match value.get_numeric_mode() {
            NumericMode::Integer => match self.calculator.state.get_numeric_base() {
                NumericBase::Decimal => format!("{}", value.get_integer()).into(),
                NumericBase::Hexadecimal => format!("{:x}", value.get_integer()).into(),
                NumericBase::Octal => format!("{:o}", value.get_integer()).into(),
                NumericBase::Binary => format!("{:b}", value.get_integer()).into(),
            }
            // untested
            NumericMode::Float => match self.calculator.state.get_numeric_base() {
                // untested
                NumericBase::Decimal => format!("{:.1$}", value.get_decimal(), self.significant_digits).into(),
                //NumericBase::Hexadecimal => format!("{}", value.get_decimal()).into(),
                NumericBase::Hexadecimal => panic!("unimplemented"),
                NumericBase::Octal => format!("{:o}", value.get_decimal().to_bits()).into(),
                NumericBase::Binary => format!("{:b}", value.get_decimal().to_bits()).into(),
            }
            // untested
            NumericMode::Scientific => match self.calculator.state.get_numeric_base() {
                // untested
                NumericBase::Decimal => {
                    let mut result: String = format!("{:e}", value.get_decimal()).into();
                    if let Some(pos) = result.find('e') {
                        let sign = result.chars().nth(pos + 1).unwrap();
                        if sign != '+' && sign != '-' {
                            result.insert(pos + 1, '+');
                        }
                    }
                    result
                }
                //NumericBase::Hexadecimal => format!("{}", value.get_decimal()).into(),
                NumericBase::Hexadecimal => panic!("unimplemented"),
                //NumericBase::Octal => format!("{:o}", value.get_decimal().to_bits()).into(),
                //NumericBase::Binary => format!("{:b}", value.get_decimal().to_bits()).into(),
                NumericBase::Octal => panic!("unimplemented"),
                NumericBase::Binary => panic!("unimplemented"),
            }
        }
    }

    fn parse_and_update_accumulator(&mut self) {
        if self.accumulator.len() == 0 {
            self.calculator.clear_accumulator();
            return
        }
        match self.calculator.get_numeric_mode() {
            NumericMode::Integer => {
                let radix: u32;
                match self.calculator.state.get_numeric_base() {
                    NumericBase::Decimal => radix = 10,
                    NumericBase::Hexadecimal => radix = 16,
                    NumericBase::Octal => radix = 8,
                    NumericBase::Binary => radix = 2,
                }
                self.calculator.set_integer_accumulator(i128::from_str_radix(&self.accumulator, radix).unwrap());
            }
            NumericMode::Float => {
                match self.calculator.state.get_numeric_base() {
                    NumericBase::Decimal => self.calculator.set_decimal_accumulator(self.accumulator.parse().unwrap()),
                    _ => panic!("unimplemented"),
                }
            }
            NumericMode::Scientific => {
                match self.calculator.state.get_numeric_base() {
                    NumericBase::Decimal => self.calculator.set_decimal_accumulator(self.accumulator.parse().unwrap()),
                    _ => panic!("unimplemented"),
                }
            }
        }
    }
}

const DISPLAY_X: u16 = 0;
const DISPLAY_Y: u16 = 0;
const DISPLAY_WIDTH: u16 = 35;
const DISPLAY_HEIGHT: u16 = 5;

impl Widget for &App {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        // A simple display area for the calculator
        let mut location = Rect{
            x: DISPLAY_X,
            y: DISPLAY_Y,
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
        };
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(location, buf);

        if let Some(error) = &self.calculator.state.get_error() {
            location.x = 1;
            location.y = 2;
            location.width = DISPLAY_WIDTH - 2;
            location.height = 1;
            Line::raw(format!("{}", ERROR_NAMES[&error]))
                .centered()
                .render(location, buf);
        } else {
            location.x = 1;
            location.y = 1;
            location.width = DISPLAY_WIDTH - 2;
            location.height = 2;
            let mut text = Text::default();
            let mut line = Line::default();
            if let Some(operation) = &self.calculator.state.get_pending_operation() {
                line.push_span(format!("{:>4} ", OPERATION_NAMES[&operation]));
            } else {
                line.push_span("     ");
            }
            if let Some(value) = self.calculator.get_accumulator() {
                line.push_span(self.format_value_pair(value));
            }
            text.push_line(line);
            text.push_line(Line::raw(self.format_value_pair(self.calculator.state.get_value())).right_aligned());
            text.render(location, buf);
        }

        // numeric base
        location = Rect{
            x: 1,
            y: 3,
            width: 8,
            height: 1,
        };
        Line::raw(format!("{}", NUMERIC_BASE_NAMES[&self.calculator.state.get_numeric_base()]))
            .render(location, buf);

        // numeric mode
        location = Rect{
            x: DISPLAY_WIDTH - 4,
            y: 3,
            width: 3,
            height: 1,
        };
        Line::raw(format!("{}", NUMERIC_MODE_NAMES[&self.calculator.get_numeric_mode()]))
            .right_aligned()
            .render(location, buf);


        // Valid keys (quick reference)
        location.x = 0;
        location.y = 5;
        location.width = DISPLAY_WIDTH;
        location.height = 20;
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(location, buf);
        // Digits
        location.x = 1;
        location.y = 6;
        location.width = DISPLAY_WIDTH - 2;
        location.height = 18;
        let mut text = Text::default();
        let mut line = Line::default();
        let mut digits: String = NUMERIC_BASE_ENTRY_KEYS[&self.calculator.state.get_numeric_base()].iter().map(|c| format!(" {c}")).collect();
        match self.calculator.get_numeric_mode() {
            NumericMode::Integer => {}
            NumericMode::Float => digits.push_str(" ."),
            NumericMode::Scientific => digits.push_str(" . E"),
        }
        line.push_span(digits);
        text.push_line(line.centered());

        // Basic Operations
        line = Line::default();
        line.push_span("+ - * / %");
        text.push_line(line.centered());

        // Numeric Base
        line = Line::default();
        line.push_span("base =>");
        let nbh_binding = NUMERIC_BASE_HELP;
        let mut bases: Vec<_> = nbh_binding.keys().collect();
        bases.sort_unstable();
        for base in bases {
            if *base != self.calculator.state.get_numeric_base() {
                let nbk_binding = NUMERIC_BASE_KEYS;
                let key = nbk_binding.iter()
                    .find(|&(_, val_base)| val_base == base)
                    .map(|(key, _)| key);
                if line.width() > 0 {
                    line.push_span("  ");
                }
                line.push_span(format!("{}: {}", key.unwrap(), NUMERIC_BASE_HELP[base]));
            }
        }
        text.push_line(line.left_aligned());

        // Numeric Mode
        line = Line::default();
        line.push_span("mode =>");
        let nmn_binding = NUMERIC_MODE_NAMES;
        let mut modes: Vec<_> = nmn_binding.keys().collect();
        modes.sort_unstable();
        for mode in modes {
            if *mode != self.calculator.get_numeric_mode() {
                let nmk_binding = NUMERIC_MODE_KEYS;
                let key = nmk_binding.iter()
                    .find(|&(_, val_mode)| val_mode == mode)
                    .map(|(key, _)| key);
                if line.width() > 0 {
                    line.push_span("  ");
                }
                line.push_span(format!("{}: {}", key.unwrap(), NUMERIC_MODE_NAMES[mode]));
            }
        }
        text.push_line(line.left_aligned());


        // Always present
        line = Line::default();
        const WIDTH: usize = 8;
        line.push_span(format!("{:<1$}", "q: quit", WIDTH));
        line.push_span(format!("{:<1$}", "?: help", WIDTH));
        text.push_line(line.centered());

        text.render(location, buf);

    }
}

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
