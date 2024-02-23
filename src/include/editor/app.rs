// vim-like
pub enum AppState {
    Insert,
    Menu,
    Visual
}
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
// Variables:
// buffer: current file content
// state: state of the editor
// cursor_line: indexed from 0, the line where the cursor is at
// cursor_letter: indexed from 0, defined where the cursor is hovering
pub struct App {
    pub buffer: String,
    pub state: AppState,
    pub cursor_line: u16,
    pub cursor_letter: u16,
    pub current_size: Rect,
}
impl App {
    pub fn new() -> App {
        App {
            buffer: String::new(),
            state: AppState::Menu,
            cursor_line: 0,
            cursor_letter: 0,
            current_size: Rect::new(0, 0, 0, 0)
        }
    }
    pub fn enable_insert_mode(&mut self) {

    }
}
