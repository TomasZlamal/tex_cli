use crate::include::editor::app::App;

pub fn handle_inputs(app: &mut App) -> bool {
    if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        //dbg!(app.current_size);
                        match key.code {
                            KeyCode::F(1) => {
                                return false;
                            },
                            KeyCode::Right => {
                                if app.cursor_letter < app.current_size.right() {
                                    app.cursor_letter += 1;
                                } 
                            },
                            KeyCode::Left => {
                                if app.cursor_letter > 0 {
                                    app.cursor_letter -= 1;
                                }
                            },
                            KeyCode::Up => {
                                if app.cursor_line > 0 {
                                    app.cursor_line -= 1;
                                }
                            },
                            KeyCode::Down => {
                                if app.cursor_line < app.current_size.bottom() {
                                    app.cursor_line += 1;
                                }
                            }
                            _ => println!("Unsupported operation"),
                        }
                    }
                    
                }
            }
    return true;
}
