use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, InputField};
use crate::input::TextInput;
use crate::ui::acl_edit::render_input;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" auth_param basic ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(inner);

    let fields: [(&str, &TextInput, InputField); 4] = [
        ("Program", &app.auth_program, InputField::AuthProgram),
        ("Children", &app.auth_children, InputField::AuthChildren),
        ("Realm", &app.auth_realm, InputField::AuthRealm),
        ("Credentials TTL", &app.auth_ttl, InputField::AuthTtl),
    ];

    for (i, (label, input, field)) in fields.iter().enumerate() {
        let focused = app.auth_field == *field;
        let style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let text = render_input(input, focused);
        let widget = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(*label)
                .border_style(style),
        );
        frame.render_widget(widget, chunks[i]);
    }

    let hint = Paragraph::new(" Tab: next field | F2: save | Esc: cancel")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(hint, chunks[4]);
}
