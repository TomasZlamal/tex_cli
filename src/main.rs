mod include;
use crate::include::Editor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    Editor::start(args);
}
