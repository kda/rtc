use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::{Line, Position, Size, Text},
};
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use std::collections::HashMap;
use std::sync::LazyLock;

mod calculator;
mod help_content;

use calculator::Calculator;
use calculator::NumericBase;
use calculator::NumericMode;
use calculator::Operation;
use calculator::Error;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Mode {
    Calculating,
    MemoryStore,
    MemoryRecall,
    SignificantDigits,
    AskingHelp,
    ShowingHelp,
}

type KeyOperation = fn(app: &mut App);

struct KeyEntry<'a> {
    mode_op: HashMap<Mode, KeyOperation>,
    hint_text: &'a str,
    help_text: &'a str,
}

const KEYS_MAPPING: LazyLock<HashMap<KeyCode, KeyEntry>> = LazyLock::new(|| {
    HashMap::from([
        (KeyCode::Esc,
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.calculator.clear(); }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                        (Mode::MemoryStore, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                        (Mode::MemoryRecall, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                        (Mode::ShowingHelp, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                    ]),
                hint_text: "clear",
                help_text: "#27",
            }
        ),
        (KeyCode::Char('?'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.mode = Mode::AskingHelp; }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                        (Mode::MemoryStore, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                        (Mode::MemoryRecall, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                    ]),
                hint_text: "help",
                help_text: "quest",
            }
        ),
        (KeyCode::Char('q'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.request_exit(); }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.request_exit(); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.request_exit(); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.request_exit(); }) as KeyOperation),
                        (Mode::ShowingHelp, (|app| { app.mode = Mode::Calculating; }) as KeyOperation),
                    ]),
                hint_text: "quit",
                help_text: "q",
            }
        ),
        (KeyCode::Char('0'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.accumulate("0"); }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(0); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(0); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(0); }) as KeyOperation),
                    ]),
                hint_text: "zero",
                help_text: "0",
            }
        ),
        (KeyCode::Char('1'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.accumulate("1"); }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(1); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(1); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(1); }) as KeyOperation),
                    ]),
                hint_text: "one",
                help_text: "1",
            }
        ),
    ])
});


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


const NUMBER_OF_REGISTERS: usize = 10;

#[derive(Debug)]
pub struct App {
    mode: Mode,
    exit_requested: bool,
    size: Size,
    calculator: Calculator,
    accumulator: String,
    memory_registers_visible: bool,
    memory_registers: [calculator::ValuePair; NUMBER_OF_REGISTERS],

    // Display oriented values
    significant_digits: usize,

    help_content: String,
}

impl App {
    fn new() -> Self {
        Self {
            mode: Mode::Calculating,
            exit_requested: false,
            size: Size::default(),
            calculator: Calculator::new(),
            accumulator: String::new(),
            memory_registers_visible: false,
            memory_registers: [calculator::ValuePair::default(); NUMBER_OF_REGISTERS],
            significant_digits: 2,
            help_content: String::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        terminal.clear()?;
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
        if let Some(ke) = KEYS_MAPPING.get(&key_event.code) {
            if let Some(ko) = ke.mode_op.get(&self.mode) {
                ko(self);
            } else if self.mode == Mode::AskingHelp {
                if let Some(help_content) = help_content::extract_key_help_content(ke.help_text) {
                    self.help_content = help_content;
                    self.mode = Mode::ShowingHelp;
                } else {
                    panic!("no help for ==>{}<==", ke.help_text);
                }
            }
            return;
        }
        match self.mode {
            Mode::Calculating => {
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
                    KeyCode::Enter | KeyCode::Char('=') => {
                        self.calculator.update_value();
                        self.accumulator.clear();
                        self.parse_and_update_accumulator();
                    },
                    KeyCode::Char('.') => {
                        if self.calculator.get_numeric_mode() != NumericMode::Integer
                                && self.accumulator.find('.').is_none() {
                            self.accumulator.push('.');
                            self.parse_and_update_accumulator();
                        }
                    }
                    KeyCode::Char('E') => {
                        // TODO: add support for display uppercase 'E' if in caps mode
                        if self.calculator.get_numeric_mode() == NumericMode::Scientific
                                && self.accumulator.find('e').is_none() {
                            self.accumulator.push_str("e+");
                            self.parse_and_update_accumulator();
                        }
                    }
                    KeyCode::Char('r') => {
                        self.mode = Mode::MemoryRecall;
                    }
                    KeyCode::Char('s') => {
                        self.mode = Mode::MemoryStore;
                    }
                    KeyCode::Char('F') => {
                        if self.calculator.get_numeric_mode() != NumericMode::Integer {
                            self.mode = Mode::SignificantDigits;
                        }
                    }
                    KeyCode::Char('M') => {
                        self.memory_registers_visible = ! self.memory_registers_visible;
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
                            for mr in self.memory_registers.iter_mut() {
                                mr.set_numeric_mode(*mode);
                            }
                        }
                        self.parse_and_update_accumulator();
                    }
                    _ => {},
                }
            }
            Mode::SignificantDigits => {
                match key_event.code {
                    KeyCode::Esc => {},
                    KeyCode::Char('?') => {},
                    KeyCode::Char('q') => self.request_exit(),
                    KeyCode::Char(c) => {
                        if let Some(number) = c.to_digit(10) {
                            if number > 0 && number <= 9 {
                                self.significant_digits = number as usize;
                            }
                        }
                    },
                    _ => {}
                }
                self.mode = Mode::Calculating;
            },
            Mode::MemoryStore => {
                match key_event.code {
                    KeyCode::Esc => {},
                    KeyCode::Char('?') => {},
                    KeyCode::Char('q') => self.request_exit(),
                    KeyCode::Char(c) => {
                        if let Some(index) = c.to_digit(10) {
                            if index > 0 && index <= 9 {
                                self.memory_registers[index as usize] = self.calculator.state.get_value();
                            }
                        }
                    },
                    _ => {}
                }
                self.mode = Mode::Calculating;
            },
            Mode::MemoryRecall => {
                match key_event.code {
                    KeyCode::Esc => {},
                    KeyCode::Char('?') => {},
                    KeyCode::Char('q') => self.request_exit(),
                    KeyCode::Char(c) => {
                        if let Some(index) = c.to_digit(10) {
                            if index > 0 && index <= 9 {
                                if self.calculator.get_numeric_mode() == NumericMode::Integer {
                                    self.accumulator = self.memory_registers[index as usize].get_integer().to_string();
                                } else {
                                    self.accumulator = self.memory_registers[index as usize].get_decimal().to_string();
                                }
                                self.parse_and_update_accumulator();
                            }
                        }
                    },
                    _ => {}
                }
                self.mode = Mode::Calculating;
            },
            _ => {},
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
                NumericBase::Decimal => {
                    format!("{:.1$}", value.get_decimal(), self.significant_digits).into()
                }
                //NumericBase::Hexadecimal => format!("{}", value.get_decimal()).into(),
                NumericBase::Hexadecimal => panic!("unimplemented"),
                NumericBase::Octal => format!("{:o}", value.get_decimal().to_bits()).into(),
                NumericBase::Binary => format!("{:b}", value.get_decimal().to_bits()).into(),
            }
            // untested
            NumericMode::Scientific => match self.calculator.state.get_numeric_base() {
                // untested
                NumericBase::Decimal => {
                    let mut result: String = format!("{:.1$e}", value.get_decimal(), self.significant_digits).into();
                    // force display of sign on exponent
                    // TODO: support uppercase 'E'
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

    fn accumulate(&mut self, s: &str) {
        self.accumulator.push_str(s);
        self.parse_and_update_accumulator();
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
                    NumericBase::Decimal => {
                        if let Ok(value) = self.accumulator.parse() {
                            self.calculator.set_decimal_accumulator(value);
                        }
                    }
                    _ => panic!("unimplemented"),
                }
            }
            NumericMode::Scientific => {
                match self.calculator.state.get_numeric_base() {
                    NumericBase::Decimal => {
                        if let Ok(value) = self.accumulator.parse() {
                            self.calculator.set_decimal_accumulator(value);
                        }
                    }
                    _ => panic!("unimplemented"),
                }
            }
        }
    }

    fn set_significant_digits(&mut self, count: usize) {
        self.significant_digits = count;
        self.mode = Mode::Calculating;
    }

    fn memory_registers_store(&mut self, index: usize) {
        self.memory_registers[index] = self.calculator.state.get_value();
        self.mode = Mode::Calculating;
    }

    fn memory_registers_recall(&mut self, index: usize) {
        if index > 9 {
            return;
        }
        if self.calculator.get_numeric_mode() == NumericMode::Integer {
            self.accumulator = self.memory_registers[index as usize].get_integer().to_string();
        } else {
            self.accumulator = self.memory_registers[index as usize].get_decimal().to_string();
        }
        self.parse_and_update_accumulator();
        self.mode = Mode::Calculating;
    }

    fn render_digits_keypad(&self, area: Rect, buf: &mut Buffer, name: &str) {
        let location = Rect {
            x:  1,
            y: DISPLAY_HEIGHT + 1,
            width: DISPLAY_WIDTH - 2,
            height: area.height - 1,
        };
        let mut text = Text::default();
        let mut line = Line::default();
        line.push_span(name);
        text.push_line(line.centered());
        line = Line::default();
        text.push_line(line);
        line = Line::default();
        let digits: String =
            NUMERIC_BASE_ENTRY_KEYS[&NumericBase::Decimal]
            .iter().map(|c| format!(" {c}")).collect();
        line.push_span(digits);
        text.push_line(line.centered());
        text.render(location, buf);
    }
}

const DISPLAY_X: u16 = 0;
const DISPLAY_Y: u16 = 0;
const DISPLAY_WIDTH: u16 = 35;
const DISPLAY_HEIGHT: u16 = 5;
const KEYPAD_X: u16 = 0;
const KEYPAD_Y: u16 = DISPLAY_HEIGHT;
const KEYPAD_WIDTH: u16 = DISPLAY_WIDTH;
const KEYPAD_HEIGHT: u16 = 20;
const MEMORY_WIDTH: u16 = 20;
const MEMORY_HEIGHT: u16 = DISPLAY_HEIGHT + KEYPAD_HEIGHT;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
            // error
            location.x = 1;
            location.y = 2;
            location.width = DISPLAY_WIDTH - 2;
            location.height = 1;
            Line::raw(format!("{}", ERROR_NAMES[&error]))
                .centered()
                .render(location, buf);
        } else {
            // accumulator and pending operation
            location.x = 1;
            location.y = 1;
            location.width = DISPLAY_WIDTH - 2;
            location.height = 8;
            let mut text = Text::default();
            let mut line = Line::default();
            if let Some(operation) = &self.calculator.state.get_pending_operation() {
                line.push_span(format!("{:>4} ", OPERATION_NAMES[&operation]));
            } else {
                line.push_span("     ");
            }
            line.push_span(&self.accumulator);
            text.push_line(line);
            text.push_line(Line::raw(self.format_value_pair(self.calculator.state.get_value())).right_aligned());
            text.render(location, buf);
        }

        // numeric base
        location = Rect{
            x: 1,
            y: DISPLAY_HEIGHT - 2,
            width: 8,
            height: 1,
        };
        Line::raw(format!("{}", NUMERIC_BASE_NAMES[&self.calculator.state.get_numeric_base()]))
            .render(location, buf);

        // numeric mode
        location = Rect{
            x: DISPLAY_WIDTH - 4,
            y: DISPLAY_HEIGHT - 2,
            width: 3,
            height: 1,
        };
        Line::raw(format!("{}", NUMERIC_MODE_NAMES[&self.calculator.get_numeric_mode()]))
            .right_aligned()
            .render(location, buf);

        // A nice box for Keypad
        location.x = KEYPAD_X;
        location.y = KEYPAD_Y;
        location.width = KEYPAD_WIDTH;
        location.height = KEYPAD_HEIGHT;
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(location, buf);

        // Valid keys (quick reference)
        match self.mode {
            Mode::Calculating => {
                // Digits
                location.x = 1;
                location.y = DISPLAY_HEIGHT + 1;
                location.width = DISPLAY_WIDTH - 2;
                location.height = area.height - 1;
                let mut text = Text::default();
                let mut line = Line::default();
                let mut digits: String =
                    NUMERIC_BASE_ENTRY_KEYS[&self.calculator.state.get_numeric_base()]
                    .iter().map(|c| format!(" {c}")).collect();
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
                        let Some(&key) = NUMERIC_BASE_KEYS.iter()
                            .find(|&(_, val_base)| val_base == base)
                            .map(|(key, _)| key) else {
                                panic!("unable to find numeric base");
                            };
                        line.push_span(format!(" {}:{}", key, NUMERIC_BASE_HELP[base]));
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
                        let Some(&key) = NUMERIC_MODE_KEYS.iter()
                                .find(|&(_, val_mode)| val_mode == mode)
                                .map(|(key, _)| key) else {
                                    panic!("unable to find numeric mode");
                                };
                        line.push_span(format!(" {key}:{}", NUMERIC_MODE_NAMES[mode]));
                    }
                }
                if self.calculator.get_numeric_mode() != NumericMode::Integer {
                    line.push_span(" F:sigdig");
                }
                text.push_line(line.left_aligned());
                text.render(location, buf);
            },
            Mode::SignificantDigits => {
                self.render_digits_keypad(area, buf, "number of significant digits");
            },
            Mode::MemoryStore => {
                self.render_digits_keypad(area, buf, "location to store value");
            },
            Mode::MemoryRecall => {
                self.render_digits_keypad(area, buf, "location to recall from");
            },
            Mode::AskingHelp => {
                // TODO: show all keys
            },
            Mode::ShowingHelp => {
                let location = Rect{
                    x: KEYPAD_X + 1,
                    y: KEYPAD_Y + 1,
                    width: KEYPAD_WIDTH - 2,
                    height: KEYPAD_HEIGHT - 4,
                };
                let mut text = Text::default();
                for help_line in self.help_content.split('\n') {
                    if help_line.len() > location.width as usize {
                        let indent = help_line.starts_with('-');
                        //let mut second_line = false;
                        let mut line = Line::default();
                        for word in help_line.split(' ') {
                            if line.width() + 1 + word.len() > location.width as usize {
                                text.push_line(line);
                                //second_line = true;
                                line = Line::raw("");
                                if indent {
                                    line.push_span(" ");
                                }
                            }
                            if line.width() > 0 {
                                line.push_span(" ");
                            }
                            line.push_span(word)
                        }
                        text.push_line(line);
                    } else {
                        text.push_line(help_line);
                    }
                }
                text.render(location, buf);
            },
        }

        // Always present
        let mut text = Text::default();
        let mut line = Line::default();
        const WIDTH: usize = 11;
        line.push_span(format!("{:<1$}", "ESC: clear", WIDTH));
        line.push_span(format!("{:^1$}", "?: help", WIDTH));
        line.push_span(format!("{:>1$}", "q: quit", WIDTH));
        text.push_line(line.centered());

        location.height = text.height() as u16;
        location.y = DISPLAY_HEIGHT + KEYPAD_HEIGHT - location.height - 1;
        text.render(location, buf);

        if self.memory_registers_visible || self.mode == Mode::MemoryStore || self.mode == Mode::MemoryRecall {
            location = Rect{
                x: DISPLAY_WIDTH,
                y: DISPLAY_Y,
                width: MEMORY_WIDTH,
                height: MEMORY_HEIGHT,
            };
            Block::bordered()
                .border_type(BorderType::Rounded)
                .render(location, buf);

            location.x += 1;
            location.y += 1;
            location.width -= 2;
            text = Text::default();
            line = Line::default();
            line.push_span("memory registers");
            text.push_line(line.centered());

            for (index, mr) in self.memory_registers.iter().enumerate() {
                line = Line::default();
                text.push_line(line);
                line = Line::default();
                line.push_span(format!("{index}: {:>}", self.format_value_pair(*mr)));
                text.push_line(line);
            }

            text.render(location, buf);
        }
    }
}

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

#[cfg(test)]
mod tests {
    use super::App;
    use super::KEYS_MAPPING;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use crate::help_content;

    struct TestApp {
        app: App,
        terminal: Terminal<TestBackend>,
    }

    impl TestApp {
        fn new() -> TestApp {
            TestApp{
                app: App::new(),
                terminal: Terminal::new(TestBackend::new(40, 26)).unwrap(),
            }
        }
        fn render(&mut self) {
            self.terminal
                .draw(|frame| frame.render_widget(&self.app, frame.area()))
                .unwrap();
        }
        fn backend(&self) -> &TestBackend {
            self.terminal.backend()
        }
        fn handle_key(&mut self, key: char) {
            let ke = KeyEvent::new(KeyCode::Char(key), KeyModifiers::NONE);
            self.app.handle_key_event(ke);
        }
        fn handle_string(&mut self, s: &str) {
            for c in s.chars() {
                self.handle_key(c);
            }
        }
    }

    #[test]
    fn initial_render_app() {
        let mut ta = TestApp::new();
        ta.render();
        assert_snapshot!(ta.backend());
    }

    #[test]
    fn quit() {
        let mut ta = TestApp::new();
        ta.handle_key('q');
        assert!(ta.app.exit_requested);
    }

    #[test]
    fn hex_base_int_mode_render() {
        let mut ta = TestApp::new();
        ta.handle_string("123=H");
        ta.render();
        assert_snapshot!(ta.backend());
    }

    #[test]
    fn dec_base_dec_mode_render() {
        let mut ta = TestApp::new();
        ta.handle_string("A123.45=+4.9F5");
        ta.render();
        assert_snapshot!(ta.backend());
    }

    #[test]
    fn help_text_for_every_key() {
        let mut help_missing = false;
        for (_, ke) in KEYS_MAPPING.iter() {
            if let Some(help) = help_content::extract_key_help_content(ke.help_text) {
                if help.is_empty() {
                    help_missing = true;
                    println!("ERROR: empty help found for =>{}<=", ke.help_text);
/* kda_COMMENTED_OUT
                } else {
                    println!("INFO: help found for =>{}<= =>{}<=", ke.help_text, help);
  kda_COMMENTED_OUT */
                }
            } else {
                help_missing = true;
                println!("ERROR: no help found for =>{}<=", ke.help_text);
            }
        }
        assert!(!help_missing);
    }
}
