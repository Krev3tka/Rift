mod app;
mod ui;
mod event;
mod api;

use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::App;

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|frame| render(frame, &app))?;

        let event = crossterm::event::read()?;

        if let crossterm::event::Event::Key(key_event) = event {
            if key_event.code == crossterm::event::KeyCode::Char('q') {
                break Ok(())
            }
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(11),
            Constraint::Min(0),
        ])
        .split(layout[0]);

    let top_area = vertical_chunks[1];

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(50),
            Constraint::Min(0),
        ])
        .split(top_area);

    let form_area = horizontal_chunks[1];

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title(" Rift login ");

    frame.render_widget(&outer_block, form_area);

    let inner_area = outer_block.inner(form_area);

    let input_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(inner_area);

    let username_block = Block::default()
        .borders(Borders::ALL)
        .title(" Login ")
        .title_alignment(Alignment::Center);

    let username_paragraph = Paragraph::new(app.username_input.as_str())
        .block(username_block);

    frame.render_widget(
        &username_paragraph,
        input_chunks[1]
    );

    let password_block = Block::default()
        .borders(Borders::ALL)
        .title(" Password ")
        .title_alignment(Alignment::Center);

    let password_paragraph = Paragraph::new(app.masterkey_input.as_str())
        .block(password_block);

    frame.render_widget(
        &password_paragraph,
        input_chunks[3]
    );

    frame.render_widget(
        Paragraph::new("q: quit")
            .block(Block::new().borders(Borders::ALL)),
        layout[1]
    );
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}
