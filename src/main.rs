mod include;
use crate::include::editor::editor::Editor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    Editor::start(args);
}
