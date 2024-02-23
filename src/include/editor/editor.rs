use std::{io, str};
use std::fs::File;
use std::io::{stdout, Write, Read, Result};
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use crate::include::editor::{ui::ui, app::App};

use super::inputs;
use super::signals::Signal;
pub struct Editor;
impl Editor {
    fn edit_file(path: &str) {
        //println!("Planning to edit file {}", file);
        let file_test = File::open(path.trim_end());
        let mut file: File;
        match file_test {
            Err(_) => {
                let res = File::create(path.trim_end());
                match res {
                    Ok(out) => {file = out;}
                    Err(err) => { 
                        println!("Failed to create file, err: {}", err);
                        println!("\nBeing given input value: ");
                        dbg!(path);
                        return;
                    }
                }
            }
            Ok(out) => {
                file = out;
            }
        }
        match Editor::interface(file) {
            Ok(_) => {},
            Err(x) => {
                println!("Error: {}", x)
            },
        }
    }
    fn interface(mut file: File) -> std::io::Result<()>{
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let buffer = match str::from_utf8(&buffer) {
            Ok(s) => s,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence")),
        };
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?; 
        let mut app = App::new();
        app.buffer = String::from(buffer);
        loop {
            terminal.draw(|f| ui(f, &mut app))?;
            match inputs::handle_inputs(&mut app) {
               Ok(signal) => {
                   match signal {
                      Signal::Quit => {
                          break;
                      },
                      Signal::Continue => { continue;},
                   }
               } 
               Err(_) => {}
            }
        }
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn start(args: Vec<String>) {
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
