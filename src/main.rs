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
    ShowError,
}

type KeyOperation = fn(app: &mut App);

#[derive(Default)]
struct KeyEntry<'a> {
    mode_op: HashMap<Mode, KeyOperation>,
    help_heading: &'a str,
    name: Option<String>,
    //hint: Option<String>,
}

const KEYS_MAPPING: LazyLock<HashMap<KeyCode, KeyEntry>> = LazyLock::new(|| {
    HashMap::from([
        (KeyCode::Backspace,
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        if app.calculator.state.get_error().is_none() {
                            if app.accumulator.len() > 0 {
                                app.accumulator.pop();
                            }
                        } else {
                            app.accumulator.clear();
                            app.calculator.clear();
                        }
                        app.parse_and_update_accumulator();
                    }) as KeyOperation),
                ]),
                //hint: "del",
                help_heading: "backspace",
                name: Some("bs".to_string()),
                ..Default::default()
            }
        ),
        (KeyCode::Enter,
            KeyEntry {
                mode_op: HashMap::from([
                     (Mode::Calculating, (|app| { app.apply_equals(); }) as KeyOperation),
                ]),
                //hint: "enter",
                help_heading: "enter",
                name: Some("ret".to_string()),
                ..Default::default()
            }
        ),
        (KeyCode::Esc,
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.accumulator.clear();
                        app.calculator.clear();
                    }) as KeyOperation),
                    (Mode::SignificantDigits, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                    (Mode::MemoryStore, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                    (Mode::MemoryRecall, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                    (Mode::ShowingHelp, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                    (Mode::ShowError, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                ]),
                //hint: "clear",
                help_heading: "escape",
                name: Some("esc".to_string()),
                ..Default::default()
            }
        ),
        (KeyCode::Char('%'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.apply_operation(Operation::Modulo); }) as KeyOperation),
                    ]),
                //hint: "mod",
                help_heading: "percent",
                ..Default::default()
            }
        ),
        (KeyCode::Char('*'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.apply_operation(Operation::Multiply); }) as KeyOperation),
                    ]),
                //hint: "mul",
                help_heading: "asterisk",
                ..Default::default()
            }
        ),
        (KeyCode::Char('+'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.apply_operation(Operation::Add); }) as KeyOperation),
                    ]),
                //hint: "add",
                help_heading: "plus",
                ..Default::default()
            }
        ),
        (KeyCode::Char('-'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.apply_operation(Operation::Subtract); }) as KeyOperation),
                    ]),
                //hint: "sub",
                help_heading: "minus",
                ..Default::default()
            }
        ),
        (KeyCode::Char('.'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        if app.calculator.get_numeric_mode() != NumericMode::Integer
                                && app.accumulator.find('.').is_none() {
                            app.accumulator.push('.');
                            app.parse_and_update_accumulator();
                        }
                    }) as KeyOperation),
                ]),
                //hint: "point",
                help_heading: "dot",
                ..Default::default()
            }
        ),
        (KeyCode::Char('/'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.apply_operation(Operation::Divide); }) as KeyOperation),
                    ]),
                //hint: "div",
                help_heading: "slash",
                ..Default::default()
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
                //hint: "zero",
                help_heading: "0",
                ..Default::default()
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
                //hint: "one",
                help_heading: "1",
                ..Default::default()
            }
        ),
        (KeyCode::Char('2'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() != NumericBase::Binary {
                                app.accumulate("2");
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(2); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(2); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(2); }) as KeyOperation),
                    ]),
                //hint: "two",
                help_heading: "2",
                ..Default::default()
            }
        ),
        (KeyCode::Char('3'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() != NumericBase::Binary {
                                app.accumulate("3");
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(3); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(3); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(3); }) as KeyOperation),
                    ]),
                //hint: "three",
                help_heading: "3",
                ..Default::default()
            }
        ),
        (KeyCode::Char('4'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() != NumericBase::Binary {
                                app.accumulate("4");
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(4); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(4); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(4); }) as KeyOperation),
                    ]),
                //hint: "four",
                help_heading: "4",
                ..Default::default()
            }
        ),
        (KeyCode::Char('5'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() != NumericBase::Binary {
                                app.accumulate("5");
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(5); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(5); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(5); }) as KeyOperation),
                    ]),
                //hint: "five",
                help_heading: "5",
                ..Default::default()
            }
        ),
        (KeyCode::Char('6'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() != NumericBase::Binary {
                                app.accumulate("6");
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(6); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(6); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(6); }) as KeyOperation),
                    ]),
                //hint: "six",
                help_heading: "6",
                ..Default::default()
            }
        ),
        (KeyCode::Char('7'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() != NumericBase::Binary {
                                app.accumulate("7");
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(7); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(7); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(7); }) as KeyOperation),
                    ]),
                //hint: "seven",
                help_heading: "7",
                ..Default::default()
            }
        ),
        (KeyCode::Char('8'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            match app.calculator.state.get_numeric_base() {
                                NumericBase::Decimal | NumericBase::Hexadecimal => app.accumulate("8"),
                                _ => {},
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(8); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(8); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(8); }) as KeyOperation),
                    ]),
                //hint: "eight",
                help_heading: "8",
                ..Default::default()
            }
        ),
        (KeyCode::Char('9'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            match app.calculator.state.get_numeric_base() {
                                NumericBase::Decimal | NumericBase::Hexadecimal => app.accumulate("9"),
                                _ => {},
                            }
                        }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| { app.set_significant_digits(9); }) as KeyOperation),
                        (Mode::MemoryStore, (|app| { app.memory_registers_store(9); }) as KeyOperation),
                        (Mode::MemoryRecall, (|app| { app.memory_registers_recall(9); }) as KeyOperation),
                    ]),
                //hint: "nine",
                help_heading: "9",
                ..Default::default()
            }
        ),
        (KeyCode::Char('='),
            KeyEntry {
                mode_op: HashMap::from([
                     (Mode::Calculating, (|app| { app.apply_equals(); }) as KeyOperation),
                ]),
                //hint: "equals",
                help_heading: "equals",
                ..Default::default()
            }
        ),
        (KeyCode::Char('?'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| { app.mode = Mode::AskingHelp; }) as KeyOperation),
                        (Mode::SignificantDigits, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                        (Mode::MemoryStore, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                        (Mode::MemoryRecall, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                        (Mode::ShowingHelp, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                        (Mode::ShowError, (|app| {app.mode = Mode::AskingHelp;}) as KeyOperation),
                    ]),
                //hint: "help",
                help_heading: "bing",
                ..Default::default()
            }
        ),
        (KeyCode::Char('A'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.set_numeric_mode(NumericMode::Float);
                    }) as KeyOperation),
                ]),
                help_heading: "A",
                ..Default::default()
            }
        ),
        (KeyCode::Char('B'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.calculator.set_numeric_base(NumericBase::Binary);
                    }) as KeyOperation),
                ]),
                //hint: "binary",
                help_heading: "B",
                ..Default::default()
            }
        ),
        (KeyCode::Char('D'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.calculator.set_numeric_base(NumericBase::Decimal);
                    }) as KeyOperation),
                ]),
                //hint: "decimal (base-10)",
                help_heading: "D",
                ..Default::default()
            }
        ),
        (KeyCode::Char('E'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        // TODO: add support for display uppercase 'E' if in caps mode
                        if app.calculator.get_numeric_mode() == NumericMode::Scientific
                                && app.accumulator.find('e').is_none() {
                            app.accumulator.push_str("e+");
                            app.parse_and_update_accumulator();
                        }
                    }) as KeyOperation),
                ]),
                //hint: "exponent",
                help_heading: "E",
                ..Default::default()
            }
        ),
        (KeyCode::Char('F'),
            KeyEntry {
                mode_op: HashMap::from([
                         (Mode::Calculating, (|app| {
                            if app.calculator.get_numeric_mode() != NumericMode::Integer {
                                app.mode = Mode::SignificantDigits;
                            }
                        }) as KeyOperation),
                ]),
                //hint: "F",
                help_heading: "F",
                ..Default::default()
            }
        ),
        (KeyCode::Char('H'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.calculator.set_numeric_base(NumericBase::Hexadecimal);
                    }) as KeyOperation),
                ]),
                //hint: "hexadecimal",
                help_heading: "H",
                ..Default::default()
            }
        ),
        (KeyCode::Char('I'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.set_numeric_mode(NumericMode::Integer);
                    }) as KeyOperation),
                ]),
                help_heading: "I",
                ..Default::default()
            }
        ),
        (KeyCode::Char('M'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            app.memory_registers_visible = ! app.memory_registers_visible;
                        }) as KeyOperation),
                ]),
                //hint: "M",
                help_heading: "M",
                ..Default::default()
            }
        ),
        (KeyCode::Char('O'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.calculator.set_numeric_base(NumericBase::Octal);
                    }) as KeyOperation),
                ]),
                //hint: "octal",
                help_heading: "O",
                ..Default::default()
            }
        ),
        (KeyCode::Char('S'),
            KeyEntry {
                mode_op: HashMap::from([
                    (Mode::Calculating, (|app| {
                        app.set_numeric_mode(NumericMode::Scientific);
                    }) as KeyOperation),
                ]),
                help_heading: "S",
                ..Default::default()
            }
        ),
        (KeyCode::Char('a'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() == NumericBase::Hexadecimal {
                                app.accumulate("a");
                            }
                        }) as KeyOperation),
                    ]),
                //hint: "a",
                help_heading: "a",
                ..Default::default()
            }
        ),
        (KeyCode::Char('b'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() == NumericBase::Hexadecimal {
                                app.accumulate("b");
                            }
                        }) as KeyOperation),
                    ]),
                //hint: "b",
                help_heading: "b",
                ..Default::default()
            }
        ),
        (KeyCode::Char('c'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() == NumericBase::Hexadecimal {
                                app.accumulate("c");
                            }
                        }) as KeyOperation),
                    ]),
                //hint: "c",
                help_heading: "c",
                ..Default::default()
            }
        ),
        (KeyCode::Char('d'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() == NumericBase::Hexadecimal {
                                app.accumulate("d");
                            }
                        }) as KeyOperation),
                    ]),
                //hint: "d",
                help_heading: "d",
                ..Default::default()
            }
        ),
        (KeyCode::Char('e'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() == NumericBase::Hexadecimal {
                                app.accumulate("e");
                            }
                        }) as KeyOperation),
                    ]),
                //hint: "e",
                help_heading: "e",
                ..Default::default()
            }
        ),
        (KeyCode::Char('f'),
            KeyEntry {
                mode_op: HashMap::from([
                        (Mode::Calculating, (|app| {
                            if app.calculator.state.get_numeric_base() == NumericBase::Hexadecimal {
                                app.accumulate("f");
                            }
                        }) as KeyOperation),
                    ]),
                //hint: "f",
                help_heading: "f",
                ..Default::default()
            }
        ),
        (KeyCode::Char('r'),
            KeyEntry {
                mode_op: HashMap::from([
                         (Mode::Calculating, (|app| { app.mode = Mode::MemoryRecall; }) as KeyOperation),
                ]),
                //hint: "r",
                help_heading: "r",
                ..Default::default()
            }
        ),
        (KeyCode::Char('s'),
            KeyEntry {
                mode_op: HashMap::from([
                         (Mode::Calculating, (|app| { app.mode = Mode::MemoryStore; }) as KeyOperation),
                ]),
                //hint: "s",
                help_heading: "s",
                ..Default::default()
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
                        (Mode::ShowError, (|app| {app.mode = Mode::Calculating;}) as KeyOperation),
                    ]),
                //hint: "quit",
                help_heading: "q",
                ..Default::default()
            }
        ),
    ])
});


// TODO: rework this to be string on left as only used for display
pub const NUMERIC_BASE_ENTRY_KEYS: LazyLock<HashMap<NumericBase, Vec<char>>> = LazyLock::new(|| {
    HashMap::from([
        (NumericBase::Decimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']),
        (NumericBase::Hexadecimal, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']),
        (NumericBase::Octal, vec!['0', '1', '2', '3', '4', '5', '6', '7']),
        (NumericBase::Binary, vec!['0', '1']),
    ])
});

// TODO: Invert this to expedite rendering hints screen
pub const NUMERIC_BASE_KEYS: LazyLock<HashMap<char, NumericBase>> = LazyLock::new(|| {
    HashMap::from([
        ('D', NumericBase::Decimal),
        ('H', NumericBase::Hexadecimal),
        ('O', NumericBase::Octal),
        ('B', NumericBase::Binary),
    ])
});

// TODO: deprecate this in favor of the key_hint field of KeyEntry.
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

// TODO: Invert this to expedite rendering hints screen
pub const NUMERIC_MODE_KEYS: LazyLock<HashMap<char, NumericMode>> = LazyLock::new(|| {
    HashMap::from([
        ('I', NumericMode::Integer),
        ('A', NumericMode::Float),
        ('S', NumericMode::Scientific),
    ])
});

// TODO: deprecate this in favor of the key_hint field of KeyEntry.
pub const NUMERIC_MODE_NAMES: LazyLock<HashMap<NumericMode, &str>> = LazyLock::new(|| {
    HashMap::from([
        (NumericMode::Integer, "int"),
        (NumericMode::Float, "dec"),
        (NumericMode::Scientific, "sci"),
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

    // updates the application's state based on user input
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
                if let Some(help_content) = help_content::extract_key_help_content(ke.help_heading) {
                    self.help_content = help_content;
                    self.mode = Mode::ShowingHelp;
                } else {
                    panic!("no help for ==>{}<==", ke.help_heading);
                }
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

    fn apply_operation(&mut self, op: Operation) {
        if let Err(_) = self.calculator.update_value() {
            self.mode = Mode::ShowError;
        }
        self.calculator.set_pending_operation(op);
        self.accumulator.clear();
        // TODO: Is this required?
        //self.parse_and_update_accumulator();
    }

    // TODO: consider merging with apply_operations
    fn apply_equals(&mut self) {
        if let Err(_) = self.calculator.update_value() {
            self.mode = Mode::ShowError;
        }
        self.accumulator.clear();
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

    fn set_numeric_mode(&mut self, mode: NumericMode) {
        self.calculator.set_numeric_mode(mode);
        for mr in self.memory_registers.iter_mut() {
            mr.set_numeric_mode(mode);
        }
        self.parse_and_update_accumulator();
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

    fn render_keypad_content(&self, _area: Rect, buf: &mut Buffer, content: &String) {
        let location = Rect{
            x: KEYPAD_X + 1,
            y: KEYPAD_Y + 1,
            width: KEYPAD_WIDTH - 2,
            height: KEYPAD_HEIGHT - 4,
        };
        let mut text = Text::default();
        for content_line in content.split('\n') {
            if content_line.len() > location.width as usize {
                let indent = content_line.starts_with('-');
                //let mut second_line = false;
                let mut line = Line::default();
                for word in content_line.split(' ') {
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
                text.push_line(content_line);
            }
        }
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

        location.x = DISPLAY_X + 1;
        location.width = DISPLAY_WIDTH - 2;
        match self.mode {
            Mode::ShowError => {
                let Some(error) = &self.calculator.state.get_error() else {
                    panic!("ERROR: display mode is ShowError, but calculator has no error");
                };
                location.y = DISPLAY_Y + 1;
                location.height = 2;
                let mut text = Text::default();
                text.push_line(Line::raw("ERROR").centered());
                text.push_line(Line::raw(format!("{}", ERROR_NAMES[&error])).centered());
                text.render(location, buf);
            }
            _ => {
                // accumulator and pending operation
                location.y = DISPLAY_Y + 1;
                location.height = 2;
                let mut text = Text::default();
                let mut line = Line::default();
                if let Some(operation) = &self.calculator.state.get_pending_operation() {
                    line.push_span(format!("{:>4} ", OPERATION_NAMES[&operation]));
                } else {
                    line.push_span("     ");
                }
                line.push_span(&self.accumulator);
                text.push_line(line);

                // current value
                text.push_line(Line::raw(self.format_value_pair(self.calculator.state.get_value())).right_aligned());
                text.render(location, buf);

                // numeric base
                location.y = DISPLAY_HEIGHT - 2;
                location.height = 1;
                Line::raw(format!("{}", NUMERIC_BASE_NAMES[&self.calculator.state.get_numeric_base()]))
                    .render(location, buf);

                // numeric mode
                location.x = DISPLAY_WIDTH - 4;
                location.width = 3;
                Line::raw(format!("{}", NUMERIC_MODE_NAMES[&self.calculator.get_numeric_mode()]))
                    .right_aligned()
                    .render(location, buf);
            }
        }

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
                let mut keys = Vec::<String>::new();
                for (key, ke) in KEYS_MAPPING.iter() {
                    if let Some(name) = &ke.name {
                        keys.push(name.clone());
                    } else {
                        if let KeyCode::Char(c) = key {
                            keys.push(c.to_string());
                        } else {
                            panic!("ERROR: key has no valid name =>{}<=", ke.help_heading);
                        }
                    }
                }
                keys.sort();
                let content = keys.join(" ");
                self.render_keypad_content(area, buf, &content);
            },
            Mode::ShowingHelp => {
                self.render_keypad_content(area, buf, &self.help_content);
            },
            Mode::ShowError => {
                // nothing to show on keypad
            },
        }

        // Nearly Always present
        if self.mode != Mode::AskingHelp {
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
        }

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
            let mut text = Text::default();
            let mut line = Line::default();
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
    fn help_heading_for_every_key() {
        let mut help_missing = false;
        for (_, ke) in KEYS_MAPPING.iter() {
            if let Some(help) = help_content::extract_key_help_content(ke.help_heading) {
                if help.is_empty() {
                    help_missing = true;
                    println!("ERROR: empty help found for =>{}<=", ke.help_heading);
/* kda_COMMENTED_OUT
                } else {
                    println!("INFO: help found for =>{}<= =>{}<=", ke.help_heading, help);
  kda_COMMENTED_OUT */
                }
            } else {
                help_missing = true;
                println!("ERROR: no help found for =>{}<=", ke.help_heading);
            }
        }
        assert!(!help_missing);
    }
}
