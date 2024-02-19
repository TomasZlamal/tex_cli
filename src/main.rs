use std::env;
use std::io;
use std::io::BufRead;
struct Editor {

}
impl Editor {
    fn start(&self, args: Vec<String>) {
        if args.len() <= 1 {
        // path with no inputted file
            println!("Input the name and path of the file");
            let mut input = &mut String::new();
            let stdin = io::stdin();
            stdin.read_line(input);
            println!("You inputted: {}", input);
            todo!();
        }
        else {
            // path with a file in the params
            let file = args.get(1);
            todo!();
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let editor: Editor = Editor{};
    editor.start(args);

    
}
