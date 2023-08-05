use crate::{
    app::{App, State},
    password::Password,
};
use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Cell, Wrap, Row, Borders, Tabs, Table, Paragraph, TableState},
    text::{Span, Spans},
    Frame,
};

pub struct View {
    passwords_table_state: TableState,
    details_table_state: TableState,
    show_password: bool,
    focus_details: bool
}

impl View {
    pub fn new() -> Self {
        View {
            passwords_table_state: TableState::default(),
            details_table_state: TableState::default(),
            show_password: false,
            focus_details: false
        }
    }

    pub fn select_next(&mut self, num_passwords: usize) {
        let (table_state, num_rows) = match self.focus_details {
            true => (&mut self.details_table_state, 4),
            false => (&mut self.passwords_table_state, num_passwords)
        };
        
        let row = match table_state.selected() {
            Some(row) => {
                if row >= num_rows - 1 {
                    0
                } else {
                    row + 1
                }
            }
            None => 0,
        };

        table_state.select(Some(row));
    }

    pub fn select_prev(&mut self, num_passwords: usize) {
        let (table_state, num_rows) = match self.focus_details {
            true => (&mut self.details_table_state, 4),
            false => (&mut self.passwords_table_state, num_passwords)
        };

        let row = match table_state.selected() {
            Some(i) => {
                if i == 0 {
                    num_rows - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        
        table_state.select(Some(row));
    }

    pub fn toggle_show_password(&mut self) {
        self.show_password = !self.show_password
    }

    pub fn toggle_focus_details(&mut self) {
        self.focus_details = !self.focus_details
    }

    pub fn draw_ui<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        app: &App
    ) -> Result<(), Box<dyn Error>> {
        let f_size = f.size();
        
        // layout for the app
        let outer_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // title
                Constraint::Length(3), // tabs
                Constraint::Min(0), // content
                Constraint::Length(1) // footer info
            ].as_ref())
            .split(f_size);
    
        // renter title
        let title_block = Block::default()
            .title("Helsafe Password Manager")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Blue));
        f.render_widget(title_block, outer_chunk[0]);

        // render tabs
        let tabs = create_tabs(app.get_tab_index());
        f.render_widget(tabs, outer_chunk[1]);

        // layout for the main content
        let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(outer_chunk[2]);
        
        // render password list
        let passwords = app.get_passwords();
        let passwords_block = create_password_table(passwords);
        f.render_stateful_widget(passwords_block, content_layout[0], &mut self.passwords_table_state);
        
        // render content based on tab
        match app.get_state() {
            State::Details => {
                let selected = match self.passwords_table_state.selected() {
                    Some(index) => index,
                    None => usize::MAX
                };

                match selected {
                    usize::MAX => {
                        let info_block = Paragraph::new("Basic instructions are located at the bottom, but additional details can be found at https://github.com/ethanhelwig/helsafe.\nThank you for using the Helsafe password manager!\n-Ethan")
                        .block(Block::default().title("Welcome").borders(Borders::ALL))
                        .wrap(Wrap{trim: true})
                        .alignment(Alignment::Left);
                        f.render_widget(info_block, content_layout[1]);
                    },
                    _ => {
                        let password = passwords.get(selected).unwrap();
                        let details_block = create_details_block(password, self.show_password);
                        f.render_stateful_widget(details_block, content_layout[1], &mut self.details_table_state);
                    }
                }
            },
            State::Insert => {
                todo!()
            },
            State::Delete => {
                todo!()
            },
            State::Search => {
                todo!()
            }
        }
    
        // render footer
        let footer_block = create_footer_block(&self.passwords_table_state, app.get_num_passwords());
        f.render_widget(footer_block, outer_chunk[3]);
    
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

fn create_tabs(tab: usize) -> Tabs<'static> {
    let titles = vec![
        Spans::from("Details"),
        Spans::from("Insert"),
        Spans::from("Delete"),
        Spans::from("Search")
    ];

    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(
            "Tabs",
            Style::default()
        )))
        .select(tab)
        .style(Style::default())
        .highlight_style(Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    )
}

fn create_details_block(password: &Password, show_password: bool) -> Table {
    let header_cells = [
        Cell::from("Type").style(Style::default()),
        Cell::from("Stored").style(Style::default())
    ];
    
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue));

    let rows = [
        Row::new(vec![
            Cell::from("Title"),
            Cell::from(password.title.clone())
        ]),
        Row::new(vec![
            Cell::from("Username"),
            Cell::from(password.username.clone())
        ]),
        Row::new(vec![
            Cell::from("Email"),
            Cell::from(password.email.clone())
        ]),
        {
            if show_password {
                Row::new(vec![
                    Cell::from("Password"),
                    Cell::from(password.password.clone())
                ])
            } else {
                Row::new(vec![
                    Cell::from("Password"),
                    Cell::from("********")
                ])
            }
        }
    ];

    Table::new(rows)
        .header(header)
        .block(Block::default().title("Details").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10)
        ])
}

fn create_footer_block(table_state: &TableState, num_passwords: usize) -> Paragraph {
    let footer_text = String::from(
        format!(
            " Selected: {}/{} (Use arrow keys to navigate) Q=Exit, Tab=Next, Insert=New password, Delete=Remove password ", 
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