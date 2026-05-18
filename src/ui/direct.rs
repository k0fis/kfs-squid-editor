use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Row, Table};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // always_direct
    let always_rows: Vec<Row> = app
        .config
        .always_direct
        .iter()
        .map(|rule| {
            let refs_str: String = rule
                .acl_refs
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            Row::new(vec![rule.action.to_string(), refs_str])
        })
        .collect();

    let widths = [Constraint::Length(6), Constraint::Fill(1)];
    let header = Row::new(vec!["Action", "ACLs"]).style(Style::default().bold().fg(Color::Cyan));

    let always_border = if app.direct_focus_always {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let always_table = Table::new(always_rows, widths)
        .header(header.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" always_direct ")
                .border_style(always_border),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_stateful_widget(always_table, chunks[0], &mut app.always_direct_state);

    // never_direct
    let never_rows: Vec<Row> = app
        .config
        .never_direct
        .iter()
        .map(|rule| {
            let refs_str: String = rule
                .acl_refs
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            Row::new(vec![rule.action.to_string(), refs_str])
        })
        .collect();

    let never_border = if !app.direct_focus_always {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let never_table = Table::new(never_rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" never_direct ")
                .border_style(never_border),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_stateful_widget(never_table, chunks[1], &mut app.never_direct_state);
}
