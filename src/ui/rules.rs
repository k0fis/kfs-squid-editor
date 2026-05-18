use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Row, Table};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    draw_acl_table(frame, app, chunks[0]);
    draw_access_table(frame, app, chunks[1]);
}

fn draw_acl_table(frame: &mut Frame, app: &mut App, area: Rect) {
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

    let border_style = if app.rules_focus_acls {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" ACLs ")
                .border_style(border_style),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_stateful_widget(table, area, &mut app.acl_table_state);
}

fn draw_access_table(frame: &mut Frame, app: &mut App, area: Rect) {
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

    let border_style = if !app.rules_focus_acls {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" http_access Rules (order matters!) ")
                .border_style(border_style),
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    frame.render_stateful_widget(table, area, &mut app.access_table_state);
}
