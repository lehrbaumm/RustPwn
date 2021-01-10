use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    while running.load(Ordering::SeqCst) {
        terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80)
                ].as_ref()
            )
            .split(f.size());
            let block = Block::default()
                .title("List")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default()
                .title("Main")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;
    }
    Ok(())
}
