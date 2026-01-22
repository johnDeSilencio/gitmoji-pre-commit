use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{ConventionalCommit, copy_to_clipboard};

const CORE: [ConventionalCommit; 6] = [
    ConventionalCommit {
        commit_type: "business",
        emoji: "👔",
        description: "Add, modify, or remove business logic",
    },
    ConventionalCommit {
        commit_type: "crit",
        emoji: "🚑️",
        description: "Critical hotfix",
    },
    ConventionalCommit {
        commit_type: "deprecate",
        emoji: "🗑️",
        description: "Changes that deprecate part or all of an API",
    },
    ConventionalCommit {
        commit_type: "extern",
        emoji: "👽",
        description: "Change resulting from external API dependency changes",
    },
    ConventionalCommit {
        commit_type: "feat",
        emoji: "✨",
        description: "Add a new feature",
    },
    ConventionalCommit {
        commit_type: "fix",
        emoji: "🩹",
        description: "Partially or completely resolve a defect or a bug",
    },
];

pub struct CoreList {
    state: ListState,
}

impl CoreList {
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
        if let Some(selected_index) = self.state.selected() && let Some(commit) = CORE.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut CoreList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Core").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(CORE.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
