pub mod io;

use crate::io::*;
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let contents = read_emojis_toml();
    let conventional_commits = parse_conventional_commits(contents);

    let app = App {
        state: AppState::Running,
        conventional_commits,
    };

    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    conventional_commits: Vec<ConventionalCommit>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? && key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => self.quit(),
                _ => {}
            }
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut commits: Vec<Line> = Vec::with_capacity(self.conventional_commits.len());

        for commit in self.conventional_commits.iter() {
            let line: Line = 
            format!("{}: {} {}\n", commit.r#type, commit.emoji, commit.description).into();
            commits.push(line);
        }

        Paragraph::new(commits).render(area, buf);
    }
}
