use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{copy_to_clipboard, ConventionalCommit};

const DEPENDENCIES: [ConventionalCommit; 4] = [
    ConventionalCommit {
        commit_type: "add",
        emoji: "➕",
        description: "Add a dependency or dependencies",
    },
    ConventionalCommit {
        commit_type: "downgrade",
        emoji: "⬇️",
        description: "Downgrade dependency or dependencies",
    },
    ConventionalCommit {
        commit_type: "remove",
        emoji: "➖",
        description: "Remove a dependency or dependencies",
    },
    ConventionalCommit {
        commit_type: "upgrade",
        emoji: "⬆️",
        description: "Upgrade dependency or dependencies",
    },
];

pub struct DependenciesList {
    state: ListState,
}

impl DependenciesList {
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
        if let Some(selected_index) = self.state.selected() && let Some(commit) = DEPENDENCIES.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut DependenciesList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Dependencies").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(DEPENDENCIES.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
