use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::app::{App, InputField};
use crate::model::AccessAction;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Edit Rule ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(inner);

    // Action selector
    let action_style = if app.edit_field == InputField::Action {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let allow_style = if app.access_action == AccessAction::Allow {
        Style::default().fg(Color::Green).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let deny_style = if app.access_action == AccessAction::Deny {
        Style::default().fg(Color::Red).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let action_line = Line::from(vec![
        Span::styled("  [ ", action_style),
        Span::styled("ALLOW", allow_style),
        Span::styled(" | ", action_style),
        Span::styled("DENY", deny_style),
        Span::styled(" ]  (Space/←/→ to toggle)", action_style),
    ]);
    let action_widget = Paragraph::new(action_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Action")
            .border_style(action_style),
    );
    frame.render_widget(action_widget, chunks[0]);

    // ACL picker — two columns
    let picker_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let available_names = app.available_acl_names();

    // Available ACLs
    let available_items: Vec<ListItem> = available_names
        .iter()
        .map(|name| ListItem::new(name.as_str()))
        .collect();

    let available_border = if app.edit_field == InputField::AclPicker && app.access_focus_available
    {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let available_list = List::new(available_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Available (Space=add)")
                .border_style(available_border),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));
    let mut available_state = ListState::default();
    if app.edit_field == InputField::AclPicker && app.access_focus_available {
        available_state.select(Some(app.access_available_cursor));
    }
    frame.render_stateful_widget(available_list, picker_chunks[0], &mut available_state);

    // Selected ACLs
    let selected_items: Vec<ListItem> = app
        .access_acl_refs
        .iter()
        .map(|acl_ref| {
            let text = if acl_ref.negated {
                format!("! {} (negated)", acl_ref.name)
            } else {
                acl_ref.name.clone()
            };
            ListItem::new(text)
        })
        .collect();

    let selected_border = if app.edit_field == InputField::AclPicker && !app.access_focus_available
    {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let selected_list = List::new(selected_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Selected (!:negate Space:remove)")
                .border_style(selected_border),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));
    let mut selected_state = ListState::default();
    if app.edit_field == InputField::AclPicker
        && !app.access_focus_available
        && !app.access_acl_refs.is_empty()
    {
        selected_state.select(Some(app.access_selected_cursor));
    }
    frame.render_stateful_widget(selected_list, picker_chunks[1], &mut selected_state);
}
