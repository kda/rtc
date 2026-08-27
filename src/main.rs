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

mod calculator;

use calculator::Calculator;
use calculator::NumericBase;
use calculator::Operation;
use calculator::Error;


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

pub const OPERATION_KEYS: LazyLock<HashMap<char, Operation>> = LazyLock::new(|| {
    HashMap::from([
        ('+', Operation::Add),
        ('-', Operation::Subtract),
        ('*', Operation::Multiply),
        ('/', Operation::Divide),
    ])
});

pub const OPERATION_NAMES: LazyLock<HashMap<Operation, &str>> = LazyLock::new(|| {
    HashMap::from([
        (Operation::Add, "+"),
        (Operation::Subtract, "-"),
        (Operation::Multiply, "*"),
        (Operation::Divide, "/"),
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
}

impl App {
    fn new() -> Self {
        Self {
            exit_requested: false,
            size: Size::default(),
            calculator: Calculator::new(),
            accumulator: String::new(),
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
                if self.calculator.state.error.is_none() {
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
            KeyCode::Char('B') => self.transition_numeric_base(NumericBase::Binary),
            KeyCode::Char('D') => self.transition_numeric_base(NumericBase::Decimal),
            KeyCode::Char('H') => self.transition_numeric_base(NumericBase::Hexadecimal),
            KeyCode::Char('O') => self.transition_numeric_base(NumericBase::Octal),
            KeyCode::Enter | KeyCode::Char('=') => {
                self.calculator.update_value();
                self.accumulator.clear();
                self.parse_and_update_accumulator();
            },
            KeyCode::Char(c) => {
                if NUMERIC_BASE_KEYS[&self.calculator.state.numeric_base].contains(&c) {
                    self.accumulator.push(c);
                } else if let Some(operation) = OPERATION_KEYS.get(&c) {
                    self.calculator.update_value();
                    self.calculator.set_pending_operation(operation.clone());
                    self.accumulator.clear();
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

    fn format_value(&self, value: i128) -> String {
        match self.calculator.state.numeric_base {
            NumericBase::Decimal => format!("{}", value).into(),
            NumericBase::Hexadecimal => format!("{:x}", value).into(),
            NumericBase::Octal => format!("{:o}", value).into(),
            NumericBase::Binary => format!("{:b}", value).into(),
        }
    }

    fn parse_and_update_accumulator(&mut self) {
        if self.accumulator.len() > 0 {
            let radix: u32;
            match self.calculator.state.numeric_base {
                NumericBase::Decimal => radix = 10,
                NumericBase::Hexadecimal => radix = 16,
                NumericBase::Octal => radix = 8,
                NumericBase::Binary => radix = 2,
            }

            self.calculator.set_accumulator(i128::from_str_radix(&self.accumulator, radix).unwrap());
        } else {
            self.calculator.clear_accumulator();
        }
    }

    fn transition_numeric_base(&mut self, numeric_base: NumericBase) {
        self.parse_and_update_accumulator();
        self.calculator.set_numeric_base(numeric_base);
        if let Some(value) = self.calculator.state.accumulator {
            self.accumulator = self.format_value(value);
        } else {
            self.accumulator.clear();
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

        if let Some(error) = &self.calculator.state.error {
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
            if let Some(operation) = &self.calculator.state.pending_operation {
                line.push_span(format!("{:>4} ", OPERATION_NAMES[&operation]));
            } else {
                line.push_span("     ");
            }
            if let Some(value) = self.calculator.state.accumulator {
                line.push_span(self.format_value(value));
            }
            text.push_line(line);
            text.push_line(Line::raw(self.format_value(self.calculator.state.value)).right_aligned());
            text.render(location, buf);
        }

        // numeric base
        location = Rect{
            x: 1,
            y: 3,
            width: 3,
            height: 1,
        };
        Line::raw(format!("{}", NUMERIC_BASE_NAMES[&self.calculator.state.numeric_base]))
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
        let digits: String = NUMERIC_BASE_KEYS[&self.calculator.state.numeric_base].iter().map(|c| format!(" {c}")).collect();
        line.push_span(digits);
        text.push_line(line.centered());
        text.render(location, buf);

    }
}

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
