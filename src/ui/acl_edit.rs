use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, InputField};
use crate::help;
use crate::model::AclType;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_form(frame, app, chunks[0]);
    draw_type_help(frame, app, chunks[1]);
}

fn draw_form(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Edit ACL ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(inner);

    // Name field
    let name_style = field_style(app.edit_field == InputField::Name);
    let name = Paragraph::new(app.edit_name.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Name")
            .border_style(name_style),
    );
    frame.render_widget(name, chunks[0]);

    // Type field
    let type_style = field_style(app.edit_field == InputField::Type);
    let current_type = &AclType::ALL[app.edit_type_index];
    let type_text = format!("< {} >", current_type);
    let type_widget = Paragraph::new(type_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Type (←/→)")
            .border_style(type_style),
    );
    frame.render_widget(type_widget, chunks[1]);

    // Values field
    let values_style = field_style(app.edit_field == InputField::Values);
    let values = Paragraph::new(app.edit_values.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Values (one per line)")
            .border_style(values_style),
    );
    frame.render_widget(values, chunks[2]);

    // Case insensitive toggle
    let ci_style = field_style(app.edit_field == InputField::CaseInsensitive);
    let ci_text = if app.edit_case_insensitive {
        "[x] Case insensitive (-i)"
    } else {
        "[ ] Case insensitive (-i)"
    };
    let ci = Paragraph::new(ci_text).style(ci_style);
    frame.render_widget(ci, chunks[3]);
}

fn draw_type_help(frame: &mut Frame, app: &App, area: Rect) {
    let current_type = &AclType::ALL[app.edit_type_index];
    let help_text = help::help_for_acl_type(current_type);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Help: {} ", current_type));
    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn field_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    }
}
