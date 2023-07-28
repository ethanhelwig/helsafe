use std::{
    io::{BufRead, BufReader},
    error::Error,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{Block, Wrap, Borders, Paragraph, Tabs},
    Frame,
};

pub fn draw_ui<B: Backend>(
    f: &mut Frame<B>, 
    app: &App, 
    size: &Rect
) -> Result<(), Box<dyn Error>> {
    todo!();
}
