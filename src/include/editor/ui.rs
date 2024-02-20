use ratatui::{prelude::*, widgets::*};
use crate::include::editor::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();
        frame.render_widget(
            Paragraph::new(app.buffer.as_str())
                .white()
                .on_black(),
            area,
        );
}
