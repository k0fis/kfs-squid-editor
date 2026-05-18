use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Row, Table};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let rows: Vec<Row> = app
        .config
        .http_access
        .iter()
        .enumerate()
        .map(|(i, rule)| {
            let refs_str: String = rule
                .acl_refs
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(" AND ");
            Row::new(vec![
                format!("{}", i + 1),
                rule.action.to_string(),
                refs_str,
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(4),
        Constraint::Length(6),
        Constraint::Fill(1),
    ];

    let header = Row::new(vec!["#", "Action", "ACLs (AND logic)"])
        .style(Style::default().bold().fg(Color::Cyan));

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" http_access Rules (order matters!) "),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_stateful_widget(table, area, &mut app.access_table_state);
}
