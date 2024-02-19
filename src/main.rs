use std::env;
use std::io;
use std::io::BufRead;
use std::fs::File;

struct Editor;
impl Editor {
    fn edit_file(path: &str) {
        //println!("Planning to edit file {}", file);
        let file_test = File::open(path.trim_end());
        let file: File;
        match file_test {
            Err(_) => {
                let res = File::create(path.trim_end());
                match res {
                    Ok(out) => {file = out;}
                    Err(err) => { 
                        println!("Failed to create file, err: {}", err);
                        println!("\nBeing given input value: ");
                        dbg!(path);
                    }
                }
            }
            Ok(out) => {
                file = out;
            }
        }
    }
    fn start(args: Vec<String>) {
        if args.len() <= 1 {
        // path with no inputted file
            println!("Input the name and path of the file");
            let input = &mut String::new();
            let stdin = io::stdin();
            let result = stdin.read_line(input);
            //println!("You inputted: {}", input);
            match result {
                Ok(_) => {
                    Editor::edit_file(input.as_str());
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
                    Editor::edit_file(x);
                }
            }
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    Editor::start(args);
}
