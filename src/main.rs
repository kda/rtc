use ratatui::{DefaultTerminal, Frame};

fn app(terminal: &mut DefaultTerminal) {
    loop {
        terminal.draw(render).expect("unable to draw terminal");
        if crossterm::event::read().expect("failed to read keyboard event").is_key_press() {
            break;
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

fn main() {
    ratatui::run(app);
}
