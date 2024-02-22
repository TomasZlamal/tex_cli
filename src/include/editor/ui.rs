use ratatui::{prelude::*, widgets::*};
use crate::include::editor::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.size();
    let cursor_style = Style::default().bg(Color::DarkGray);
    let paragraph = Paragraph::new(app.buffer.as_str())
                .white()
                .on_black();

    app.current_size = Rect::new(0, 0, area.right()-1, (app.buffer.lines().count()-1) as u16);
    frame.render_widget(
        paragraph,
        area,
    );
    frame.buffer_mut().get_mut(app.cursor_letter, app.cursor_line).set_style(cursor_style);
}
