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
    let groups = parse_groups(contents.clone());
    let conventional_commits = parse_conventional_commits(contents);

    let app = App {
        state: AppState::Running,
        current_group_index: 0,
        current_subgroup_index: None,
        groups,
        conventional_commits,
    };

    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    current_group_index: usize,
    current_subgroup_index: Option<usize>,
    groups: Vec<Group>,
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
                KeyCode::Char('h') | KeyCode::Left => self.move_left(),
                KeyCode::Char('l') | KeyCode::Right => self.move_right(),
                KeyCode::Char('j') | KeyCode::Down => self.move_down(),
                KeyCode::Char('k') | KeyCode::Up => self.move_up(),
                KeyCode::Enter => self.handle_enter(),
                KeyCode::Char('q') => self.quit(),
                _ => {}
            }
        }

        Ok(())
    }

    fn move_left(&mut self) {
        todo!()
    }

    fn move_right(&mut self) {
        todo!()
    }

    fn move_up(&mut self) {
        if self.current_group_index == 0 {
            self.current_group_index = self.groups.len() - 1;
        } else {
            self.current_group_index -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.current_group_index + 1 == self.groups.len() {
            self.current_group_index = 0;
        } else {
            self.current_group_index += 1;
        }
    }
    
    fn handle_enter(&mut self) {
        todo!()
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut groups: Vec<Line> = Vec::with_capacity(self.groups.len());
        
        for (index, group) in self.groups.iter().enumerate() {
            let mut line: Line = format!("{}: {}", group.name, group.description).into();
        
            if self.current_group_index == index {
                line = line.bg(Color::Green);
            }

            groups.push(line);
        }
            
        let mut commits: Vec<Line> = Vec::with_capacity(self.conventional_commits.len());

        for commit in self.conventional_commits.iter() {
            let line: Line = 
            format!("{}: {} {}\n", commit.r#type, commit.emoji, commit.description).into();
            commits.push(line);
        }

        Paragraph::new(groups).render(area, buf);
        // Paragraph::new(commits).render(area, buf);
    }
}
