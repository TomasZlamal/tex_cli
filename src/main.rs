use std::env;
use std::io;
use std::io::BufRead;
struct Editor {

}
impl Editor {
    fn edit_file(&self, file: &str) {

    }
    fn start(&self, args: Vec<String>) {
        if args.len() <= 1 {
        // path with no inputted file
            println!("Input the name and path of the file");
            let mut input = &mut String::new();
            let stdin = io::stdin();
            let result = stdin.read_line(input);
            //println!("You inputted: {}", input);
            match result {
                Ok(output) => {
                    self.edit_file(input.as_str());
                }
                Err(output) => {
                    println!("File input failed, stopping process");
                    println!("Error info: {}", output);
                    return;
                }
            }
        }
        else {
            // path with a file in the params
            let file = args.get(1);
            match file {
                None => {
                    println!("Process failed, shutting down");
                    return;
                }
                Some(x) =>{
                    self.edit_file(x.as_str());
                }
            }
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let editor: Editor = Editor{};
    editor.start(args);
}
