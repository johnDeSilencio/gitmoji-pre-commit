pub mod emoji;

use std::io;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

use crate::emoji::{MainList, ScreenMode};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App {
        exit: false,
        screen_mode: ScreenMode::Main(MainList::new()),
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result
}

struct App {
    exit: bool,
    screen_mode: ScreenMode,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('j') => match &mut self.screen_mode {
                    ScreenMode::Main(list) => list.select_next(),
                    ScreenMode::Accessibility(list) => list.select_next(),
                    ScreenMode::Architecture(list) => list.select_next(),
                    ScreenMode::Core(list) => list.select_next(),
                    ScreenMode::Cybersecurity(list) => list.select_next(),
                    ScreenMode::Dependencies(list) => list.select_next(),
                    ScreenMode::Deployment(list) => list.select_next(),
                    ScreenMode::Documentation(list) => list.select_next(),
                    ScreenMode::Fun(list) => list.select_next(),
                    ScreenMode::Impermanent(list) => list.select_next(),
                    ScreenMode::Improvement(list) => list.select_next(),
                    ScreenMode::Infrastructure(list) => list.select_next(),
                    ScreenMode::Metadata(list) => list.select_next(),
                    ScreenMode::Persistence(list) => list.select_next(),
                    ScreenMode::Presentation(list) => list.select_next(),
                    ScreenMode::Testing(list) => list.select_next(),
                },
                KeyCode::Char('k') => match &mut self.screen_mode {
                    ScreenMode::Main(list) => list.select_previous(),
                    ScreenMode::Accessibility(list) => list.select_previous(),
                    ScreenMode::Architecture(list) => list.select_previous(),
                    ScreenMode::Core(list) => list.select_previous(),
                    ScreenMode::Cybersecurity(list) => list.select_previous(),
                    ScreenMode::Dependencies(list) => list.select_previous(),
                    ScreenMode::Deployment(list) => list.select_previous(),
                    ScreenMode::Documentation(list) => list.select_previous(),
                    ScreenMode::Fun(list) => list.select_previous(),
                    ScreenMode::Impermanent(list) => list.select_previous(),
                    ScreenMode::Improvement(list) => list.select_previous(),
                    ScreenMode::Infrastructure(list) => list.select_previous(),
                    ScreenMode::Metadata(list) => list.select_previous(),
                    ScreenMode::Persistence(list) => list.select_previous(),
                    ScreenMode::Presentation(list) => list.select_previous(),
                    ScreenMode::Testing(list) => list.select_previous(),
                },
                KeyCode::Esc => match &mut self.screen_mode {
                    ScreenMode::Main(_) => self.exit = true,
                    _ => self.screen_mode = ScreenMode::Main(MainList::new()),
                },
                KeyCode::Enter => match &mut self.screen_mode {
                    ScreenMode::Main(list) => {
                        if let Some(screen_mode) = list.select() {
                            self.screen_mode = screen_mode;
                        }
                    }
                    ScreenMode::Accessibility(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Architecture(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Core(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Cybersecurity(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Dependencies(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Deployment(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Documentation(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Fun(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Impermanent(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Improvement(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Infrastructure(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Metadata(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Persistence(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Presentation(list) => {
                        list.select();
                        self.exit = true;
                    }
                    ScreenMode::Testing(list) => {
                        list.select();
                        self.exit = true;
                    }
                },
                _ => {}
            }
        }

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
    where
        Self: Sized,
    {
        match &mut self.screen_mode {
            ScreenMode::Main(list) => list.render(area, buf),
            ScreenMode::Accessibility(list) => list.render(area, buf),
            ScreenMode::Architecture(list) => list.render(area, buf),
            ScreenMode::Core(list) => list.render(area, buf),
            ScreenMode::Cybersecurity(list) => list.render(area, buf),
            ScreenMode::Dependencies(list) => list.render(area, buf),
            ScreenMode::Deployment(list) => list.render(area, buf),
            ScreenMode::Documentation(list) => list.render(area, buf),
            ScreenMode::Fun(list) => list.render(area, buf),
            ScreenMode::Impermanent(list) => list.render(area, buf),
            ScreenMode::Improvement(list) => list.render(area, buf),
            ScreenMode::Infrastructure(list) => list.render(area, buf),
            ScreenMode::Metadata(list) => list.render(area, buf),
            ScreenMode::Persistence(list) => list.render(area, buf),
            ScreenMode::Presentation(list) => list.render(area, buf),
            ScreenMode::Testing(list) => list.render(area, buf),
        }
    }
}
