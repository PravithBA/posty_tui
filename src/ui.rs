use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

use crate::app::{Pane, State};

pub fn ui(f: &mut Frame, state: &mut State) {
    let main_layout = Layout::default()
        .constraints([
            Constraint::Percentage(6),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());
    let title_area = main_layout[0];
    let body_area = main_layout[1];
    let footer_area = main_layout[2];

    let selected_pane_in_string = match state.selected_pane {
        Pane::Index => "Index",
        Pane::ContentUrl => "Url",
        Pane::ContentBody => "Body",
    };

    let title_content = format!("Posty | {}", selected_pane_in_string);

    f.render_widget(
        Paragraph::new(Text::styled(
            title_content,
            Style::default().fg(Color::Green),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default()),
        ),
        title_area,
    );

    generate_body_area(f, body_area, state);

    f.render_widget(
        Paragraph::new(Text::styled("Footer", Style::default().fg(Color::Green))).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default()),
        ),
        footer_area,
    );
}

fn generate_body_area(f: &mut Frame, body_area: Rect, state: &mut State) {
    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Min(0)])
        .split(body_area);
    let index_area = body_layout[0];
    let content_area = body_layout[1];
    handle_index_area(f, index_area, state);
    handle_content_area(f, content_area, state);
}

fn handle_index_area(f: &mut Frame, index_area: Rect, state: &mut State) {
    let title = match state.selected_pane {
        Pane::Index => "Index | Is Active",
        _ => "Index",
    };

    let block = match state.selected_pane {
        Pane::Index => Block::default()
            .borders(Borders::ALL)
            .fg(Color::Green)
            .title(title),
        _ => Block::default().borders(Borders::ALL),
    };

    let mut list_items = Vec::<ListItem>::new();

    let offset = state.index_list_state.offset() as f32;
    let content_height = (state.requests.len()) as f32;
    let viewport_height = index_area.height as f32;
    let max_offset = content_height - viewport_height;
    let max_sroll = state.requests.len() as f32;
    let vertical_scroll = if max_offset > 0.0 {
        (offset / max_offset * max_sroll).min(max_sroll)
    } else {
        0.0
    };

    for (index, request) in state.requests.iter().enumerate() {
        let style = if state.index_list_state.selected() == Some(index) {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Yellow)
        };

        let text = format!("{}) {}", index, request);

        list_items.push(ListItem::new(Line::from(Span::styled(text, style))));
    }

    let list = List::new(list_items).highlight_symbol("> ");

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"));

    let mut scrollbar_state = ScrollbarState::new(list.len()).position(vertical_scroll as usize);

    f.render_stateful_widget(list.block(block), index_area, &mut state.index_list_state);

    f.render_stateful_widget(
        scrollbar,
        index_area.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}

fn handle_content_area(f: &mut Frame, content_area: Rect, state: &State) {
    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(content_area);
    let content_url_area = content_layout[0];
    let content_body_area = content_layout[1];

    handle_content_url_area(f, content_url_area, state);
    handle_content_body_area(f, content_body_area, state);
}

fn handle_content_url_area(f: &mut Frame, content_url_area: Rect, state: &State) {
    let title = "Url";
    let block = match state.selected_pane {
        Pane::ContentUrl => Block::default()
            .borders(Borders::ALL)
            .fg(Color::Green)
            .title(title),
        _ => Block::default().borders(Borders::ALL),
    };
    f.render_widget(
        Paragraph::new(Text::styled(
            state.url.clone(),
            Style::default().fg(Color::Yellow),
        ))
        .block(block),
        content_url_area,
    );
}

fn handle_content_body_area(f: &mut Frame, content_body_area: Rect, state: &State) {
    match state.selected_pane {
        Pane::ContentBody => {
            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(Color::Green)
                    .title("Body"),
                content_body_area,
            );
        }
        _ => {
            f.render_widget(Block::default().borders(Borders::ALL), content_body_area);
        }
    }
}
