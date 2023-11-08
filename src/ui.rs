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
    let height = app.game.map.map_state_matrix.height;
    let cell_height = (100. / height as f64) as usize;

    let width = app.game.map.map_state_matrix.width;
    let cell_width = (100. / width as f64) as usize;

    let top_margin = ((100 - cell_height * height) / 2) as u16;
    let bottom_margin = ((100 - cell_height * height) / 2) as u16;

    let left_margin = ((100 - cell_width * width) / 2) as u16;
    let right_margin = ((100 - cell_width * width) / 2) as u16;

    let mut row_constraints = Vec::new();
    row_constraints.append(&mut vec![Constraint::Percentage(top_margin)]);
    row_constraints.append(
            &mut std::iter::repeat(Constraint::Percentage(cell_height as u16))
            .take(height)
            .collect::<Vec<_>>()
        );
    row_constraints.append(&mut vec![Constraint::Percentage(bottom_margin)]);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(layout);

    for i in 0..height {
        let mut column_constraints = Vec::new();
        column_constraints.append(&mut vec![Constraint::Percentage(right_margin)]);
        column_constraints.append(
                &mut std::iter::repeat(Constraint::Percentage(cell_width as u16))
                .take(width)
                .collect::<Vec<_>>()
            );
        column_constraints.append(&mut vec![Constraint::Percentage(left_margin)]);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(column_constraints)
            .split(vertical_layout[i + 1]);

        for j in 0..width {
            frame.render_widget(
                Paragraph::new("")
                .block(
                    Block::default()
                        .title(format!("[{}, {}]", i, j))
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                    )
                .style(Style::default().fg(
                    match app.game.map.map_state_matrix.matrix[i][j].cell_type {
                        crate::core::map::matrix::cell::CellType::Wall => Color::Blue,
                        crate::core::map::matrix::cell::CellType::Pathway => 
                            match app.game.map.map_state_matrix.matrix[i][j].cell_presence {
                                crate::core::map::matrix::cell::CellPresence::Pacman => Color::Yellow,
                                crate::core::map::matrix::cell::CellPresence::Ghost => Color::Magenta,
                                crate::core::map::matrix::cell::CellPresence::None => 
                                    match app.game.map.map_state_matrix.matrix[i][j].cell_modificator {
                                        crate::core::map::matrix::cell::CellModificator::Point => Color::White,
                                        crate::core::map::matrix::cell::CellModificator::Bonus => Color::Cyan,
                                        crate::core::map::matrix::cell::CellModificator::Super => Color::Red,
                                        crate::core::map::matrix::cell::CellModificator::None => Color::Gray,
                                    },
                            },
                    }
                ))
                .alignment(Alignment::Center),
                horizontal_layout[j + 1]
            );
        }
    }
}
