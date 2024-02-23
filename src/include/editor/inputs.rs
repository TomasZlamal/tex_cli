use crate::include::editor::{app::App, signals::Signal, app::AppState};
use ratatui::prelude::{CrosstermBackend, Terminal};
use crossterm::{*, event::*};
use std::io::*;

pub fn handle_inputs(app: &mut App) -> Result<Signal> {
    match app.state {
        AppState::Menu => {
            return handle_menu_events(app);
        },
        AppState::Insert => {
            return handle_insert_events(app);
        },
        AppState::Visual => {
            return handle_visual_events(app);
        }
    }
}

fn handle_insert_events(app: &mut App) -> Result<Signal> {
    todo!();
    return Ok(Signal::Continue);
}
fn handle_visual_events(app: &mut App) -> Result<Signal> {
    todo!();
    return Ok(Signal::Continue);
}
fn handle_menu_events(app: &mut App) -> Result<Signal> {
    if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        //dbg!(app.current_size);
                        match key.code {
                            KeyCode::F(1) => {
                                return Ok(Signal::Quit); 
                            },
                            KeyCode::Right => {
                                if app.cursor_letter < app.current_size.right() {
                                    app.cursor_letter += 1;
                                } 
                            },
                            KeyCode::Left => {
                                if app.cursor_letter > 0 {
                                    app.cursor_letter -= 1;
                                }
                            },
                            KeyCode::Up => {
                                if app.cursor_line > 0 {
                                    app.cursor_line -= 1;
                                }
                            },
                            KeyCode::Down => {
                                if app.cursor_line < app.current_size.bottom() {
                                    app.cursor_line += 1;
                                }
                            }
                            _ => {},
                        }
                    }
                    
                }
            }
    return Ok(Signal::Continue);
}
