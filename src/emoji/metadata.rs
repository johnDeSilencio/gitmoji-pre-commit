use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{copy_to_clipboard, ConventionalCommit};

const METADATA: [ConventionalCommit; 9] = [
    ConventionalCommit {
        commit_type: "change",
        emoji: "📋",
        description: "Modifications to the changelog",
    },
    ConventionalCommit {
        commit_type: "contributor",
        emoji: "👥",
        description: "Modifications to the list of contributors",
    },
    ConventionalCommit {
        commit_type: "flag",
        emoji: "🚩",
        description: "Add, modify, or remove feature flags",
    },
    ConventionalCommit {
        commit_type: "ignore",
        emoji: "🙈",
        description: "Modify ignore files like .gitignore",
    },
    ConventionalCommit {
        commit_type: "init",
        emoji: "🎉",
        description: "Initial commit",
    },
    ConventionalCommit {
        commit_type: "license",
        emoji: "📄",
        description: "Modifications to the license of the project or repo",
    },
    ConventionalCommit {
        commit_type: "merge",
        emoji: "🔀",
        description: "Merge branches",
    },
    ConventionalCommit {
        commit_type: "move",
        emoji: "🚚",
        description: "Move files or resources to alternate path",
    },
    ConventionalCommit {
        commit_type: "revert",
        emoji: "⏪",
        description: "Revert a commit",
    },
];

pub struct MetadataList {
    state: ListState,
}

impl MetadataList {
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
        if let Some(selected_index) = self.state.selected() && let Some(commit) = METADATA.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut MetadataList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Metadata").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(METADATA.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
