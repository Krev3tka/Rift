mod app;
mod ui;
mod event;
mod api;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::{App, InputField};
use ratatui::style::{Color, Style};

fn main() -> color_eyre::Result<()> {
    let mut application = App::new();
    color_eyre::install()?;
    ratatui::run(|terminal| { app(terminal, &mut application) })?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &app))?;

        let event = crossterm::event::read()?;

        if let crossterm::event::Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Esc =>  {
                        break Ok(())
                    }
                    KeyCode::Tab => {
                        app.active_field = match app.active_field {
                            InputField::Username => InputField::Masterkey,
                            InputField::Masterkey => InputField::Username,
                        }
                    }
                    KeyCode::Backspace => {
                        match app.active_field {
                            InputField::Username => { app.username_input.pop(); }
                            InputField::Masterkey => { app.masterkey_input.pop(); }
                        }
                    }
                    KeyCode::Char(c) => {
                        match app.active_field {
                            InputField::Username => { app.username_input.push(c); }
                            InputField::Masterkey => { app.masterkey_input.push(c); }
                        }
                    }
                    KeyCode::Enter => {
                        // api logic

                        app.username_input.clear();
                        app.masterkey_input.clear();
                        app.active_field = InputField::Username;
                    }
                    _ => {}
                }
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

    let username_style = if app.active_field == InputField::Username {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let username_block = Block::default()
        .borders(Borders::ALL)
        .title(" Login ")
        .title_alignment(Alignment::Center)
        .border_style(username_style);

    let username_paragraph = Paragraph::new(app.username_input.as_str())
        .block(username_block);

    frame.render_widget(
        &username_paragraph,
        input_chunks[1]
    );

    let password_style = if app.active_field == InputField::Masterkey {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let password_block = Block::default()
        .borders(Borders::ALL)
        .title(" Password ")
        .title_alignment(Alignment::Center)
        .border_style(password_style);

    let password_paragraph = Paragraph::new(app.masterkey_input.as_str())
        .block(password_block);

    frame.render_widget(
        &password_paragraph,
        input_chunks[3]
    );

    frame.render_widget(
        Paragraph::new(" Esc: quit | Tab: switch field ")
            .block(Block::new().borders(Borders::ALL)),
        layout[1]
    );
}
