use crate::{
    app::App,
    password::Password,
};
use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Cell, Row, Borders, Table, Paragraph, TableState},
    text::Span,
    Frame,
};

pub struct View {
    table_state: TableState
}

impl View {
    pub fn new() -> Self {
        View {
            table_state: TableState::default()
        }
    }

    pub fn select_next(&mut self, num_passwords: usize) {
        let row = match self.table_state.selected() {
            Some(row) => {
                if row >= num_passwords - 1 {
                    0
                } else {
                    row + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(row));
    }

    pub fn select_prev(&mut self, num_passwords: usize) {
        let row = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    num_passwords - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(row));
    }

    pub fn draw_ui<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        app: &App
    ) -> Result<(), Box<dyn Error>> {
        let f_size = f.size();
    
        /* Outer-most layout for:
         * - Title
         * - Content
         * - Footer
         */
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1)
            ].as_ref())
            .split(f_size);
    
        // renter title
        let title_block = Block::default()
            .title("Helsafe Password Manager")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Rgb(200,200,200)));
        f.render_widget(title_block, outer_layout[0]);
       
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(outer_layout[1]);
        
        let passwords = app.get_passwords();
        let password_list = create_password_table(passwords); 
        f.render_stateful_widget(password_list, content_layout[0], &mut self.table_state);
    
        let footer_block = create_footer_block(app);
        f.render_widget(footer_block, outer_layout[2]);
    
        Ok(())
    }
}

fn create_password_table(passwords: &Vec<Password>) -> Table {
    let header_cells = [
        Cell::from("Title").style(Style::default().fg(Color::Rgb(200,200,200))),
        Cell::from("Username/Email").style(Style::default().fg(Color::Rgb(200,200,200))),
    ];
    
    let header = Row::new(header_cells)
        .style(Style::default().fg(Color::Rgb(200,200,200)))
        .height(1)
        .bottom_margin(1);

    let mut rows: Vec<Row> = Vec::new();
    for password in passwords.iter() {
        let mut cells: Vec<Cell> = Vec::new();
        cells.push(Cell::from(password.title.clone()));
        if !password.username.is_empty() && !password.email.is_empty() {
            cells.push(
                Cell::from(password.email.clone())
                    .style(Style::default().fg(Color::Rgb(200,200,200)))
            );
        }
        else if password.username.is_empty() {
            cells.push(
                Cell::from(password.email.clone())
                    .style(Style::default().fg(Color::Rgb(200,200,200)))
            );
        }
        else {
            cells.push(
                Cell::from(password.username.clone())
                    .style(Style::default().fg(Color::Rgb(200,200,200)))
            );
        }

        rows.push(
            Row::new(cells)
                .height(1)
                .bottom_margin(1)
        )
    }

    Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Passwords"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
}

fn create_footer_block(app: &App) -> Paragraph {
    let footer_block = Block::default();
    let footer_text = String::from(format!("test"));
    
    Paragraph::new(Span::styled(footer_text, Style::default()))
}
