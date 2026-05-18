use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::app::{App, Tab};
use crate::help;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let popup_width = (area.width * 70 / 100).max(40);
    let popup_height = (area.height * 70 / 100).max(10);
    let popup_area = super::centered_rect(popup_width, popup_height, area);

    let screen_name = match app.tab {
        Tab::Rules => "rules",
        Tab::Auth => "auth",
        Tab::Direct => "direct",
    };
    let help_text = help::help_for_screen(screen_name);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Help (? or Esc to close) ");

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, popup_area);
    frame.render_widget(paragraph, popup_area);
}
