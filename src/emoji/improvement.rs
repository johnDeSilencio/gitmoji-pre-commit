use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{copy_to_clipboard, ConventionalCommit};

const IMPROVEMENT: [ConventionalCommit; 6] = [
    ConventionalCommit {
        commit_type: "dead",
        emoji: "⚰️",
        description: "Remove dead code",
    },
    ConventionalCommit {
        commit_type: "handle",
        emoji: "🥅",
        description: "Improve error handling",
    },
    ConventionalCommit {
        commit_type: "lint",
        emoji: "🚨",
        description: "Fix linter warnings / errors",
    },
    ConventionalCommit {
        commit_type: "perf",
        emoji: "⚡",
        description: "Improve performance of existing function / feature",
    },
    ConventionalCommit {
        commit_type: "refactor",
        emoji: "♻️",
        description: "Modify existing behavior",
    },
    ConventionalCommit {
        commit_type: "style",
        emoji: "🎨",
        description: "Change whitespace and / or formatting",
    },
];

pub struct ImprovementList {
    state: ListState,
}

impl ImprovementList {
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
        if let Some(selected_index) = self.state.selected() && let Some(commit) = IMPROVEMENT.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut ImprovementList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Improvement").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(IMPROVEMENT.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
