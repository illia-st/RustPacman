use tui::{
    layout::{Alignment, self},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, canvas::Canvas},
    Frame, prelude::{Layout, Direction, Constraint, Rect},
};

use crate::app::App;

const CELL_WIDTH: u16 = 2;
const CELL_HEIGHT: u16 = 2;

/// Renders the user interface widgets.
#[allow(unused_variables)]
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(75),
            Constraint::Percentage(15)
        ])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new("")
        .block(
            Block::default()
                .title("Top")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
          )
          .style(Style::default().fg(Color::Yellow))
          .alignment(Alignment::Center),
        outer_layout[0]);

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5)
        ])
        .split(outer_layout[1]);

    frame.render_widget(
        Paragraph::new("")
        .block(
            Block::default()
                .title("Left")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
            inner_layout[0]);

    frame.render_widget(
        Paragraph::new("")
        .block(
            Block::default()
                .title("Right")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
            inner_layout[2]);

    frame.render_widget(
        Paragraph::new("")
        .block(
            Block::default()
                .title("Bottom")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
          )
          .style(Style::default().fg(Color::Yellow))
          .alignment(Alignment::Center),
          outer_layout[2]);

    render_game_field(app, frame, inner_layout[1]);
}

fn render_game_field(app: &mut App, frame: &mut Frame, layout: Rect) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Pacman"))
        .marker(tui::symbols::Marker::Braille)
        .x_bounds([0., (app.game.map.screen_width * app.game.map.tile_width) as f64])
        .y_bounds([0., (app.game.map.screen_height * app.game.map.tile_height) as f64])
        .paint(|ctx| {
            ctx.draw(&app.game.map);
        });
    
    frame.render_widget(canvas, layout);

    // println!("H:[{}], W:[{}]", layout.height, layout.width)
}
