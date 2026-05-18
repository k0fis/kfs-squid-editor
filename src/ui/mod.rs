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
            Screen::List => format!(
                " Tab:panel  Esc:next tab  a:add  e:edit  d:del  u/J:move  ?:help  Ctrl+s:save  q:quit{dirty}"
            ),
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
