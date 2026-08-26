use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::{Line, Position, Size},
};
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;
use ratatui::widgets::Widget;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;

mod calculator;

use calculator::Calculator;

/* kda_COMMENTED_OUT
#[derive(Clone, Debug, Default)]
pub struct Display {
    //widgets: Vec<dyn Widget>,
}

impl Widget for Display {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let output = format!("w: {} h: {}", self.size.width, self.size.height);
        frame.render_widget(output, frame.area());
    }
}
  kda_COMMENTED_OUT */

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
            //width: 80,
            //height: 80,
            size: Size::default(),
            //widgets: Vec::new(),
            //display: Display::default(),
            calculator: Calculator::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        /* kda_COMMENTED_OUT
        println!("yo, world!"));
        println!("terminal.size(): {:?}", terminal.size());
        return Ok(());
          kda_COMMENTED_OUT */
        self.size = terminal.size()?;

        while !self.exit_requested {
            terminal.draw(|frame| self.draw(frame))?;
						self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        //let output = format!("w: {} h: {}", self.size.width, self.size.height);
        //frame.render_widget(output, frame.area());
        /* kda_COMMENTED_OUT
        for widget in self.widgets {
            frame.render_widget(*widget, frame.area());
        }
          kda_COMMENTED_OUT */
        //frame.render_widget(self.display.clone(), frame.area());
        /* kda_COMMENTED_OUT
        frame.render_widget(Display::default(), frame.area());
          kda_COMMENTED_OUT */
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
            KeyCode::Char('q') => self.request_exit(),
            KeyCode::Enter => self.calculator.update_value(),
            KeyCode::Char(c) => {
                if calculator::NUMERIC_BASE_KEYS[&self.calculator.state.numeric_base].contains(&c) {
                    self.calculator.accumulate(c);
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
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        // Accumulator
        location.x = 1;
        location.y = 1;
        location.width = 28;
        location.height = 1;
        Line::raw(format!("{}", self.calculator.state.accumulator))
            .left_aligned()
            .render(location, buf);

        // Current value
        location.x = 0;
        location.y = 2;
        location.width = 28;
        location.height = 1;
        Line::raw(format!("{}", self.calculator.state.value))
            .right_aligned()
            .render(location, buf);

        // numeric base
        let mut location = Rect{
            x: 1,
            y: 3,
            width: 3,
            height: 1,
        };
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(location, buf);
        Line::raw(format!("{}", calculator::NUMERIC_BASE_NAMES[&self.calculator.state.numeric_base]))
            .render(location, buf);
    }
}

fn main() -> std::io::Result<()> {
    //println!("hello, world!");
    ratatui::run(|terminal| App::new().run(terminal))
}
