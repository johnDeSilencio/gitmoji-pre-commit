use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{copy_to_clipboard, ConventionalCommit};

const DOCUMENTATION: [ConventionalCommit; 3] = [
    ConventionalCommit {
        commit_type: "comment",
        emoji: "📖",
        description: "Modifications to doc comments",
    },
    ConventionalCommit {
        commit_type: "docs",
        emoji: "📝",
        description: "Add, edit or remove user-facing documentation",
    },
    ConventionalCommit {
        commit_type: "typo",
        emoji: "✏️",
        description: "Fix typos",
    },
];

pub struct DocumentationList {
    state: ListState,
}

impl DocumentationList {
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
        if let Some(selected_index) = self.state.selected() && let Some(commit) = DOCUMENTATION.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut DocumentationList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Documentation").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(DOCUMENTATION.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
