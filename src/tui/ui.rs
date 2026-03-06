use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph},
};

use super::app::App;

pub fn draw(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let msg_count = app.messages().len();
    let refresh_msg = if app.is_refreshing() { 1 } else { 0 };
    let msg_height = if msg_count + refresh_msg == 0 { 0 } else { (msg_count + refresh_msg) as u16 + 2 };

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(msg_height),
        Constraint::Length(1),
    ])
    .split(area);

    draw_header(frame, app, chunks[0]);
    draw_stations(frame, app, chunks[1]);
    if msg_height > 0 {
        draw_messages(frame, app, chunks[2]);
    }
    draw_help_bar(frame, chunks[3], app.has_imports());

    if app.show_help() {
        draw_help_popup(frame, area, app.has_imports());
    }
}

fn draw_header(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let state = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async { app.state().lock().await.clone() })
    });

    let station_name = match app.playing_index() {
        Some(idx) => format!(" {} ", app.stations()[idx].name),
        None => " Not Playing ".to_string(),
    };

    let volume_text = if state.muted {
        format!(" Muted({}) ", state.volume)
    } else {
        format!(" Vol {} ", state.volume)
    };

    let status_style = Style::default()
        .fg(Color::Rgb(36, 36, 36))
        .bg(Color::Rgb(255, 95, 135));

    let track_style = Style::default()
        .fg(Color::Rgb(36, 36, 36))
        .bg(Color::Rgb(147, 147, 255));

    let volume_style = Style::default()
        .fg(Color::Rgb(36, 36, 36))
        .bg(Color::Rgb(165, 80, 223));

    let header_area = Rect::new(area.x + 1, area.y + 1, area.width.saturating_sub(2), 1);

    let status_width = station_name.len() as u16;
    let volume_width = volume_text.len() as u16;
    let track_width = header_area.width.saturating_sub(status_width + volume_width);

    let columns = Layout::horizontal([
        Constraint::Length(status_width),
        Constraint::Length(track_width),
        Constraint::Length(volume_width),
    ])
    .split(header_area);

    let status = Paragraph::new(station_name)
        .style(status_style)
        .alignment(ratatui::layout::Alignment::Left);
    frame.render_widget(status, columns[0]);

    let track_display = if state.current_track.is_empty() {
        " ".repeat(track_width as usize)
    } else {
        state.current_track.clone()
    };
    let track = Paragraph::new(track_display)
        .style(track_style)
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(track, columns[1]);

    let volume = Paragraph::new(volume_text)
        .style(volume_style)
        .alignment(ratatui::layout::Alignment::Right);
    frame.render_widget(volume, columns[2]);
}

fn draw_stations(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let stations = app.stations();
    let cursor = app.cursor();
    let playing_index = app.playing_index();

    let page_height = area.height.saturating_sub(2) as usize;
    let scroll_offset = if cursor >= page_height {
        (cursor - page_height / 2).min(stations.len().saturating_sub(page_height))
    } else {
        0
    };

    let items: Vec<ListItem> = stations
        .iter()
        .skip(scroll_offset)
        .take(page_height)
        .enumerate()
        .map(|(i, station)| {
            let idx = scroll_offset + i;
            let is_selected = idx == cursor;
            let is_playing = playing_index == Some(idx);
            let is_remote = station.is_remote;

            let prefix = if is_selected { "▶ " } else { "  " };
            let remote_indicator = if is_remote { " ☁" } else { "" };

            let (prefix_style, name_style) = if is_playing && is_selected {
                (
                    Style::default().fg(Color::Rgb(255, 95, 135)).add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::Rgb(255, 95, 135)).add_modifier(Modifier::BOLD),
                )
            } else if is_playing {
                (
                    Style::default().fg(Color::Rgb(255, 95, 135)),
                    Style::default().fg(Color::Rgb(255, 95, 135)).add_modifier(Modifier::ITALIC),
                )
            } else if is_selected {
                (
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    Style::default().fg(Color::DarkGray),
                    Style::default().fg(Color::Gray),
                )
            };

            let remote_style = Style::default().fg(Color::Rgb(100, 149, 237));

            let line = Line::from(vec![
                Span::styled(prefix, prefix_style),
                Span::styled(&station.name, name_style),
                Span::styled(remote_indicator, remote_style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(80, 80, 100)))
            .title(Span::styled(
                format!(" Stations ({}) ", stations.len()),
                Style::default().fg(Color::Rgb(147, 147, 255)).bold(),
            ))
            .padding(Padding::uniform(1)),
    );

    frame.render_widget(list, area);
}

fn draw_messages(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    if app.is_refreshing() {
        lines.push(Line::styled(
            " Refreshing remote station lists... ",
            Style::default().fg(Color::Rgb(147, 147, 255)).bg(Color::Rgb(40, 40, 80)),
        ));
    }

    for m in app.messages() {
        if m.is_error() {
            lines.push(Line::styled(
                format!(" {} ", m.content()),
                Style::default().fg(Color::Rgb(255, 100, 100)).bg(Color::Rgb(60, 40, 40)),
            ));
        } else {
            lines.push(Line::styled(
                format!(" {} ", m.content()),
                Style::default().fg(Color::Rgb(150, 255, 150)).bg(Color::Rgb(40, 60, 40)),
            ));
        }
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(80, 80, 100))),
    );
    frame.render_widget(paragraph, area);
}

fn draw_help_bar(frame: &mut ratatui::Frame, area: Rect, has_imports: bool) {
    let mut shortcuts: Vec<(&str, &str)> = vec![
        ("?", "Help"),
        ("Enter", "Play"),
        ("m", "Mute"),
        ("*/+", "Vol+"),
        ("-/\\", "Vol-"),
    ];

    if has_imports {
        shortcuts.push(("r", "Refresh"));
    }

    shortcuts.push(("q", "Quit"));

    let spans: Vec<Span> = shortcuts
        .iter()
        .flat_map(|(key, action)| {
            vec![
                Span::styled(
                    format!(" {} ", key),
                    Style::default()
                        .fg(Color::Rgb(36, 36, 36))
                        .bg(Color::Rgb(147, 147, 255)),
                ),
                Span::styled(
                    format!("{} ", action),
                    Style::default().fg(Color::DarkGray),
                ),
            ]
        })
        .collect();

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn draw_help_popup(frame: &mut ratatui::Frame, area: Rect, has_imports: bool) {
    let popup_width = 65.min(area.width);
    let popup_height = if has_imports { 20 } else { 18 }.min(area.height);
    let popup_area = Rect::new(
        (area.width.saturating_sub(popup_width)) / 2,
        (area.height.saturating_sub(popup_height)) / 2,
        popup_width,
        popup_height,
    );

    frame.render_widget(Clear, popup_area);

    let key_style = Style::default()
        .fg(Color::Rgb(36, 36, 36))
        .bg(Color::Rgb(147, 147, 255));

    let mut help_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ↑/k  ", key_style),
            Span::raw(" Move up       "),
            Span::styled(" Enter ", key_style),
            Span::raw(" Play station  "),
        ]),
        Line::from(vec![
            Span::styled("  ↓/j  ", key_style),
            Span::raw(" Move down     "),
            Span::styled("   m   ", key_style),
            Span::raw(" Toggle mute   "),
        ]),
        Line::from(vec![
            Span::styled("  ←/h  ", key_style),
            Span::raw(" Page left     "),
            Span::styled("  +/-  ", key_style),
            Span::raw(" Volume        "),
        ]),
        Line::from(vec![
            Span::styled("  →/l  ", key_style),
            Span::raw(" Page right    "),
            Span::raw("                "),
        ]),
        Line::from(""),
    ];

    if has_imports {
        help_lines.push(Line::from(vec![
            Span::styled("   r   ", key_style),
            Span::raw(" Refresh remote stations"),
        ]));
        help_lines.push(Line::from(""));
    }

    help_lines.push(Line::from(vec![
        Span::styled("  q/Esc ", key_style),
        Span::raw(" Quit           "),
        Span::styled("   ?   ", key_style),
        Span::raw(" Toggle help   "),
    ]));
    help_lines.push(Line::from(""));

    let paragraph = Paragraph::new(help_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(147, 147, 255)))
            .title(Span::styled(
                " ⚙ Keybindings ",
                Style::default().fg(Color::Rgb(255, 95, 135)).bold(),
            )),
    );
    frame.render_widget(paragraph, popup_area);
}
