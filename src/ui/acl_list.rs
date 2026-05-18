use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Row, Table};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let rows: Vec<Row> = app
        .config
        .acls
        .iter()
        .map(|acl| {
            let ci = if acl.case_insensitive { "-i" } else { "" };
            let vals = if acl.values.len() > 3 {
                format!(
                    "{} ... (+{})",
                    acl.values[..3].join(" "),
                    acl.values.len() - 3
                )
            } else {
                acl.values.join(" ")
            };
            Row::new(vec![
                acl.name.clone(),
                acl.acl_type.to_string(),
                ci.to_string(),
                vals,
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(16),
        Constraint::Length(14),
        Constraint::Length(3),
        Constraint::Fill(1),
    ];

    let header = Row::new(vec!["Name", "Type", "Fl", "Values"])
        .style(Style::default().bold().fg(Color::Cyan));

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" ACLs "))
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_stateful_widget(table, area, &mut app.acl_table_state);
}
