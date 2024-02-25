use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Wrap,
    },
    Frame,
};

use crate::app::{Pane, Popup, State};

pub fn ui(f: &mut Frame, state: &mut State) {
    let main_layout = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());
    let title_area = main_layout[0];
    let body_area = main_layout[1];
    let footer_area = main_layout[2];

    let title_content = format!(
        "Posty | {} | {}",
        state.selected_pane.to_string(),
        state.mode.to_string(),
    );

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

    if let Some(popup) = &state.popup {
        match popup {
            Popup::CreateRequest => handle_create_request_popup(f, state),
        };
    }
}

fn handle_create_request_popup(f: &mut Frame, state: &mut State) {
    let text = if let Some(selected_request) = state.get_selected_request() {
        Text::from(selected_request.label.clone())
    } else {
        Text::from("")
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Enter Request Name (Esc - Cancel | Enter - Create)");

    f.render_widget(
        Paragraph::new(text.alignment(Alignment::Center)).block(block),
        get_centered_rect_from_length(60, 3, f.size()),
    )
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
    let title = "Index";
    let block = match state.selected_pane {
        Pane::Index => Block::default()
            .borders(Borders::ALL)
            .fg(Color::Green)
            .title(title),
        _ => Block::default().borders(Borders::ALL),
    };

    if state.requests.is_empty() {
        let text =
            Text::from("Please press 'c' to create a new request!").alignment(Alignment::Center);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(13), Constraint::Min(0)])
            .split(index_area);

        f.render_widget(
            Paragraph::new(text).wrap(Wrap::default()),
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Min(10),
                    Constraint::Length(2),
                ])
                .split(layout[1])[1],
        );
        f.render_widget(block, index_area);
        return;
    }

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

        let text = format!("{}) {}", index, request.label);

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
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(content_url_area);
    let content_method_area = content_layout[0];
    let content_url_text_area = content_layout[1];

    handle_content_url_method_area(f, content_method_area, state);
    handle_content_url_text_area(f, content_url_text_area, state);
}

fn handle_content_url_text_area(f: &mut Frame, content_url_text_area: Rect, state: &State) {
    let title = Pane::ContentUrl.to_string();
    let block = match state.selected_pane {
        Pane::ContentUrl => Block::default()
            .borders(Borders::ALL)
            .fg(Color::Green)
            .title(title),
        _ => Block::default().borders(Borders::ALL),
    };
    let text: String = if let Some(selected_request_index) = state.index_list_state.selected() {
        state.requests[selected_request_index].url.clone()
    } else {
        "".into()
    };
    f.render_widget(
        Paragraph::new(Text::styled(text, Style::default().fg(Color::Yellow))).block(block),
        content_url_text_area,
    );
}

fn handle_content_url_method_area(f: &mut Frame, content_url_method_area: Rect, state: &State) {
    let title = Pane::ContentMethod.to_string();
    let block = match state.selected_pane {
        Pane::ContentMethod => Block::default()
            .borders(Borders::ALL)
            .fg(Color::Green)
            .title(title),
        _ => Block::default().borders(Borders::ALL),
    };
    let text: String = if let Some(selected_request_index) = state.index_list_state.selected() {
        state.requests[selected_request_index].method.to_string()
    } else {
        "".into()
    };
    f.render_widget(
        Paragraph::new(Text::styled(text, Style::default().fg(Color::Yellow))).block(block),
        content_url_method_area,
    );
}

fn handle_content_body_area(f: &mut Frame, content_body_area: Rect, state: &State) {
    match state.selected_pane {
        Pane::ContentBody => {
            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(Color::Green)
                    .title(Pane::ContentBody.to_string()),
                content_body_area,
            );
        }
        _ => {
            f.render_widget(Block::default().borders(Borders::ALL), content_body_area);
        }
    }
}

fn get_centered_rect_from_length(length_x: u16, length_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - length_y) / 2),
            Constraint::Length(length_y),
            Constraint::Length((r.height - length_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width - length_x) / 2),
            Constraint::Length(length_x),
            Constraint::Length((r.width - length_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
