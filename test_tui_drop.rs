use crossterm::{terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}, execute, event::{DisableMouseCapture, EnableMouseCapture}};

fn main() {
    let _ = execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
    let _ = disable_raw_mode();
    let _ = enable_raw_mode();
    let _ = execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture);
}
