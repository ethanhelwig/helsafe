mod app;
mod view;
mod backend;
use app::App;
use view::View;
use backend::Backend;
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

use std::{
    error::Error,
    io,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // prompt password from user
    let pass_key = rpassword::prompt_password("Enter password:")?;
    
    // app setup
    let mut app = App::new(&pass_key)?; // success dependant on pass key

    // terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run app
    let res = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }
    
    Ok(())
}
