use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{copy_to_clipboard, ConventionalCommit};

const INFRASTRUCTURE: [ConventionalCommit; 4] = [
    ConventionalCommit {
        commit_type: "build",
        emoji: "👷",
        description: "Change build process",
    },
    ConventionalCommit {
        commit_type: "chore",
        emoji: "🔧",
        description: "Modifications to the list of contributors",
    },
    ConventionalCommit {
        commit_type: "ci",
        emoji: "🧱",
        description: "Change CI/CD build pipeline",
    },
    ConventionalCommit {
        commit_type: "devx",
        emoji: "🖥️",
        description: "Improve or modify developer experience",
    },
];

pub struct InfrastructureList {
    state: ListState,
}

impl InfrastructureList {
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
        if let Some(selected_index) = self.state.selected() && let Some(commit) = INFRASTRUCTURE.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut InfrastructureList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Infrastructure").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(INFRASTRUCTURE.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
