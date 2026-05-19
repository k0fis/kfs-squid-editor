mod access_edit;
mod acl_edit;
mod auth;
mod direct;
mod help_popup;
mod rules;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::app::{ALL_TABS, App, Screen, Tab};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(frame.area());

    draw_tabs(frame, app, chunks[0]);

    match &app.screen {
        Screen::List => match app.tab {
            Tab::Rules => rules::draw(frame, app, chunks[1]),
            Tab::Auth => auth::draw(frame, app, chunks[1]),
            Tab::Direct => direct::draw(frame, app, chunks[1]),
        },
        Screen::AclEdit { .. } => acl_edit::draw(frame, app, chunks[1]),
        Screen::AccessEdit { .. } | Screen::DirectEdit { .. } => {
            access_edit::draw(frame, app, chunks[1]);
        }
        Screen::ConfirmQuit => {
            draw_content_for_tab(frame, app, chunks[1]);
            draw_confirm(frame, "Unsaved changes! Quit? (y/n)", chunks[1]);
        }
        Screen::ConfirmDelete => {
            draw_content_for_tab(frame, app, chunks[1]);
            draw_confirm(frame, "Delete this item? (y/n)", chunks[1]);
        }
    }

    draw_status_bar(frame, app, chunks[2]);

    if app.menu_active {
        draw_menu(frame, app, chunks[0]);
    }

    if app.help_visible {
        help_popup::draw(frame, app);
    }
}

fn draw_content_for_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.tab {
        Tab::Rules => rules::draw(frame, app, area),
        Tab::Auth => auth::draw(frame, app, area),
        Tab::Direct => direct::draw(frame, app, area),
    }
}

fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<&str> = ALL_TABS.iter().map(|t| t.title()).collect();
    let selected = ALL_TABS.iter().position(|t| t == &app.tab).unwrap_or(0);

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" kfs-squid-editor "),
        )
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow).bold());

    frame.render_widget(tabs, area);
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let msg = if let Some((ref text, _)) = app.status_message {
        text.clone()
    } else {
        let dirty = if app.dirty { " [modified]" } else { "" };
        match &app.screen {
            Screen::List => {
                format!(" F9:menu  a:add  e:edit  d:del  /:search  Ctrl+z:undo  ?:help{dirty}")
            }
            Screen::AclEdit { .. } | Screen::AccessEdit { .. } | Screen::DirectEdit { .. } => {
                format!(" Tab:field  F2:save  Esc:cancel{dirty}")
            }
            _ => dirty.to_string(),
        }
    };

    let style = if app.status_message.is_some() {
        Style::default().fg(Color::Green).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let bar = Paragraph::new(msg).style(style);
    frame.render_widget(bar, area);
}

fn draw_confirm(frame: &mut Frame, msg: &str, area: Rect) {
    let popup_area = centered_rect(40, 5, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .title(" Confirm ");
    let text = Paragraph::new(msg)
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(ratatui::widgets::Clear, popup_area);
    frame.render_widget(text, popup_area);
}

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

fn draw_menu(frame: &mut Frame, app: &App, tab_area: Rect) {
    let menus = app.menu_items();

    // Draw menu bar (overwrite tab bar area)
    let mut spans = Vec::new();
    spans.push(Span::raw(" "));
    for (i, (title, _)) in menus.iter().enumerate() {
        let style = if i == app.menu_index {
            Style::default().bg(Color::Yellow).fg(Color::Black).bold()
        } else {
            Style::default().fg(Color::White)
        };
        spans.push(Span::styled(format!(" {title} "), style));
    }
    let bar = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::DarkGray))
        .block(Block::default());
    frame.render_widget(bar, Rect::new(tab_area.x, tab_area.y, tab_area.width, 1));

    // Draw dropdown below the selected menu title
    let (_, items) = menus[app.menu_index];
    let dropdown_width: u16 = items
        .iter()
        .map(|(label, shortcut)| label.len() + shortcut.len() + 4)
        .max()
        .unwrap_or(10) as u16
        + 2;
    let dropdown_height = items.len() as u16 + 2;

    // Calculate x position of the dropdown
    let mut x_offset: u16 = 1;
    for (i, (title, _)) in menus.iter().enumerate() {
        if i == app.menu_index {
            break;
        }
        x_offset += title.len() as u16 + 2;
    }

    let dropdown_area = Rect::new(
        tab_area.x + x_offset,
        tab_area.y + 1,
        dropdown_width.min(tab_area.width.saturating_sub(x_offset)),
        dropdown_height,
    );

    let rows: Vec<Line> = items
        .iter()
        .enumerate()
        .map(|(i, (label, shortcut))| {
            let style = if i == app.menu_item {
                Style::default().bg(Color::Yellow).fg(Color::Black)
            } else {
                Style::default().fg(Color::White)
            };
            let padding = dropdown_width as usize - label.len() - shortcut.len() - 4;
            let text = format!(" {label}{:>width$}{shortcut} ", "", width = padding);
            Line::from(Span::styled(text, style))
        })
        .collect();

    let dropdown = Paragraph::new(rows).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::DarkGray)),
    );
    frame.render_widget(ratatui::widgets::Clear, dropdown_area);
    frame.render_widget(dropdown, dropdown_area);
}
