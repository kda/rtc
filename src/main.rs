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
}

impl App {
    fn new() -> Self {
        Self {
            exit_requested: false,
            size: Size::default(),
            calculator: Calculator::new(),
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
                    self.calculator.unaccumulate();
                } else {
                    self.calculator.clear();
                }
            }
            KeyCode::Esc => self.calculator.clear(),
            KeyCode::Char('q') => self.request_exit(),
            KeyCode::Enter | KeyCode::Char('=') => self.calculator.update_value(),
            KeyCode::Char(c) => {
                if NUMERIC_BASE_KEYS[&self.calculator.state.numeric_base].contains(&c) {
                    self.calculator.accumulate(c);
                } else if let Some(operation) = OPERATION_KEYS.get(&c) {
                    self.calculator.update_value();
                    self.calculator.set_pending_operation(operation.clone());
                }
            }
            _ => {
            }
        }
    }

    fn request_exit(&mut self) {
        self.exit_requested = true;
    }

}

impl Widget for &App {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        // A simple display area for the calculator
        let mut location = Rect{
            x: 0,
            y: 0,
            width: 30,
            height: 5,
        };
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(location, buf);

        if let Some(error) = &self.calculator.state.error {
            location.x = 1;
            location.y = 2;
            location.width = 29;
            location.height = 1;
            Line::raw(format!("{}", ERROR_NAMES[&error]))
                .centered()
                .render(location, buf);
        } else {
            location.x = 1;
            location.y = 1;
            location.width = 28;
            location.height = 2;
            let mut text = Text::default();
            let mut line = Line::default();
            if let Some(operation) = &self.calculator.state.pending_operation {
                line.push_span(format!("{:>4} ", OPERATION_NAMES[&operation]));
            } else {
                line.push_span("     ");
            }
            line.push_span(&self.calculator.state.accumulator);
            text.push_line(line);
            text.push_line(Line::raw(format!("{}", self.calculator.state.value)).right_aligned());
            text.render(location, buf);
        }

        // numeric base
        location = Rect{
            x: 1,
            y: 3,
            width: 3,
            height: 1,
        };
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(location, buf);
        Line::raw(format!("{}", NUMERIC_BASE_NAMES[&self.calculator.state.numeric_base]))
            .render(location, buf);

        // Valid keys (quick reference)
        // Digits

    }
}

fn main() -> std::io::Result<()> {
    //println!("hello, world!");
    ratatui::run(|terminal| App::new().run(terminal))
}
