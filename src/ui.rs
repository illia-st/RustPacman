use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
#[allow(unused_variables)]
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!("This is a tui template.\n Press `Esc`, `Ctrl-C` or `q` to stop running.\nPacman:[({}, {})]\nGhost[1]:[({}, {})], Ghost[2]:[({}, {})], Ghost[3]:[({}, {})], Ghost[4]:[({}, {})]",
        app.game.map.map_graph.graph[app.game.pacman.curr_cell].x,
        app.game.map.map_graph.graph[app.game.pacman.curr_cell].y,
        app.game.map.map_graph.graph[app.game.ghosts[0].curr_cell].x,
        app.game.map.map_graph.graph[app.game.ghosts[0].curr_cell].y,
        app.game.map.map_graph.graph[app.game.ghosts[1].curr_cell].x,
        app.game.map.map_graph.graph[app.game.ghosts[1].curr_cell].y,
        app.game.map.map_graph.graph[app.game.ghosts[2].curr_cell].x,
        app.game.map.map_graph.graph[app.game.ghosts[2].curr_cell].y,
        app.game.map.map_graph.graph[app.game.ghosts[3].curr_cell].x,
        app.game.map.map_graph.graph[app.game.ghosts[3].curr_cell].y
    ))
            .block(
                Block::default()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
            frame.size(),
    )
}
