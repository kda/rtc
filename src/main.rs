use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    exit_requested: bool,
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

fn main() -> std::io::Result<()> {
    ratatui::run(app)
}
