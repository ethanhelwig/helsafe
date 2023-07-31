use crate::{
    app::App,
    password::Password,
};
use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Cell, Row, Borders, Table},
    Frame,
};

pub fn draw_ui<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
) -> Result<(), Box<dyn Error>> {
    let f_size = f.size();

    let title_block = Block::default()
        .title("Helsafe Password Manager")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Rgb(200,200,200)));
    f.render_widget(title_block, f_size);
   
    let parent_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f_size);
    
    let password_list = create_password_table(&app.passwords); 
    f.render_widget(password_list, parent_chunk[0]);

    Ok(())
}

fn create_password_table(passwords: &Vec<Password>) -> Table {
    let header_cells = ["Title", "Username/Email"].iter().map(|h| Cell::from(*h).style(Style::default().fg(Color::Rgb(200,200,200))));

    let header = Row::new(header_cells)
        .style(Style::default().fg(Color::Rgb(200,200,200)))
        .height(1)
        .bottom_margin(1);

    let rows = passwords.iter().map(|pw| {
        let pw_vec = pw.into_vec();
        let height = pw_vec
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = pw_vec.iter().map(|c| Cell::from(*c.clone()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Passwords"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
}
