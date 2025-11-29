#![allow(non_snake_case)]
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{Result, stdout};
mod cursor;
use cursor::Editor;
// --- End of declarations ------------------------------------------

fn main() -> Result<()> {
    enable_raw_mode()?; // TODO: handle results better
    execute!(stdout(), EnterAlternateScreen)?; // *same problem as above

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut editor = Editor::new();

    // Main event loop
    loop {
        // Render UI
        terminal.draw(|frame| {
            let size = frame.area();

            let paragraph = Paragraph::new(editor.content()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("TommyTeditor"),
            );

            frame.render_widget(paragraph, size);

            // Cursor positioning (convert bytes into column)
            let cursor_x = editor
                .cursor()
                .column as u16;
            let cursor_y = editor.cursor().line as u16;

            frame.set_cursor_position(Position {
                x: cursor_x + 1,
                y: cursor_y + 1,
            });
        })?;

        // Handle Input
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => editor.insert_char(c),
                    KeyCode::Backspace => editor.backspace(),
                    KeyCode::Left => editor.move_left(),
                    KeyCode::Right => editor.move_right(),
                    KeyCode::Enter => editor.enter(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
