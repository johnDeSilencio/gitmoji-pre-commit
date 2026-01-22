use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{copy_to_clipboard, ConventionalCommit};

pub const TESTING: [ConventionalCommit; 3] = [
    ConventionalCommit {
        commit_type: "mock",
        emoji: "🤡",
        description: "Changes relating to mocks for unit tests",
    },
    ConventionalCommit {
        commit_type: "pass",
        emoji: "✅",
        description: "Update previously passing tests or pass previously failing tests",
    },
    ConventionalCommit {
        commit_type: "test",
        emoji: "🧪",
        description: "Add one or more failing tests",
    },
];

pub struct TestingList {
    state: ListState,
}

impl TestingList {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
        }
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select(&self) {
        if let Some(selected_index) = self.state.selected() && let Some(commit) = TESTING.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut TestingList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Testing").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(TESTING.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
