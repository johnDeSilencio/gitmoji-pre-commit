use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};

use crate::emoji::{copy_to_clipboard, ConventionalCommit};

pub const DEPLOYMENT: [ConventionalCommit; 4] = [
    ConventionalCommit {
        commit_type: "deploy",
        emoji: "🚀",
        description: "Relates to deployment",
    },
    ConventionalCommit {
        commit_type: "health",
        emoji: "🩺",
        description: "Changes relating to detecting the health of the deployment",
    },
    ConventionalCommit {
        commit_type: "logging",
        emoji: "🪵",
        description: "Modify features related to logging",
    },
    ConventionalCommit {
        commit_type: "metrics",
        emoji: "📈",
        description: "Metrics or analytics",
    },
];

pub struct DeploymentList {
    state: ListState,
}

impl DeploymentList {
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
        if let Some(selected_index) = self.state.selected() && let Some(commit) = DEPLOYMENT.get(selected_index) {
            copy_to_clipboard(format!("{}: {}", commit.commit_type, commit.emoji));
        }
    }
}

impl Widget for &mut DeploymentList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Deployment").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(DEPLOYMENT.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
