use std::{env, io, str};
use std::fs::File;
use std::io::{stdout, Write, Read, Result};
use std::thread;
use std::time::Duration;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
struct Editor;
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
        Editor::crossterm_interface(file);
    }
    fn crossterm_interface(mut file: File) -> std::io::Result<()>{
        /*let mut stdout = stdout();
        stdout.queue(terminal::Clear(ClearType::All));
        stdout.flush();*/
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer);
        let buffer = match str::from_utf8(&buffer) {
            Ok(s) => s,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence")),
        };
       /*execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            //SetBackgroundColor(Color::Red),
            Print(buffer),
            ResetColor
        )?; 
                stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        stdout.flush();*/
        stdout().execute(EnterAlternateScreen);
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?; 
        loop {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
            })?;
            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('q')
                    {
                        break;
                    }
                }
            }
        }
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
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
