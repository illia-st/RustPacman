use std::io;

use pacman::app::App;
use pacman::app::AppResult;

use pacman::core::map::graph::graph::MapGraph;
use pacman::core::map::matrix::matrix::MapMatrix;
use pacman::event::Event;
use pacman::event::EventHandler;

use pacman::handler::handle_key_events;

use pacman::tui::Tui;

use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    let matrix = MapMatrix::load_matrix_from_file("/home/tr3tiakoff/University/RustPacman/res/default_map.txt");
    println!("{:#?}", matrix);

    let graph = MapGraph::loag_graph_from_matrix(&matrix);
    println!("{:#?}", graph);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
