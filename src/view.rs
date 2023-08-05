use crate::{
    app::App,
    password::Password,
};
use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
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
    
        let footer_block = create_footer_block(&self.table_state, passwords.len());
        f.render_widget(footer_block, outer_layout[2]);
    
        Ok(())
    }
}

fn create_password_table(passwords: &Vec<Password>) -> Table {
    let header_cells = [
        Cell::from("Title").style(Style::default()),
        Cell::from("Username/Email").style(Style::default())
    ];
    
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue));

    let rows = passwords.iter().map(|item| {
        let title = &item.title;
        let username = {
            if !item.username.is_empty() && !item.email.is_empty() {
                &item.username
            }
            else if !item.username.is_empty() {
                &item.username
            }
            else {
                &item.email
            }
        };

        let cells = vec![
            Cell::from(title.clone()),
            Cell::from(username.clone())
        ];

        Row::new(cells)
    });

    Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Passwords"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10)
        ]
    )
}

fn create_footer_block(table_state: &TableState, num_passwords: usize) -> Paragraph {
    
    let footer_text = String::from(
        format!(
            " selected: {}/{} (use arrow keys to navigate, press q to exit) ", 
            match table_state.selected() {
                Some(index) => index + 1,
                None => 0
            }, 
            num_passwords
        )
    );

    Paragraph::new(Span::styled(
        footer_text, 
        Style::default().add_modifier(Modifier::REVERSED))
    )
}
