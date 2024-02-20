// vim-like
pub enum AppState {
    Insert,
    Menu,
    Visual
}
pub struct App {
    pub buffer: String,
    state: AppState
}
impl App {
    pub fn new() -> App {
        App {
            buffer: String::new(),
            state: AppState::Menu
        }
    }
    pub fn enable_insert_mode(&mut self) {

    }
}
