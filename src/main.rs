mod editor;
use editor::Editor;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    Editor::start(args);
}
