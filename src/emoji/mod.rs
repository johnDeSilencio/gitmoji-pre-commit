pub mod accessibility;
pub mod architecture;
pub mod core;
pub mod cybersecurity;
pub mod dependencies;
pub mod deployment;
pub mod documentation;
pub mod fun;
pub mod impermanent;
pub mod improvement;
pub mod infrastructure;
pub mod metadata;
pub mod persistence;
pub mod presentation;
pub mod testing;

use std::fmt;

use accessibility::*;
use architecture::*;
use core::*;
use cybersecurity::*;
use dependencies::*;
use deployment::*;
use documentation::*;
use fun::*;
use impermanent::*;
use improvement::*;
use infrastructure::*;
use metadata::*;
use persistence::*;
use presentation::*;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget};
use testing::*;

pub struct ConventionalCommit {
    commit_type: &'static str,
    emoji: &'static str,
    description: &'static str,
}

impl fmt::Display for ConventionalCommit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} --> {}",
            self.commit_type, self.emoji, self.description
        )
    }
}

pub enum ScreenMode {
    Main(MainList),
    Accessibility(AccessibilityList),
    Architecture(ArchitectureList),
    Core(CoreList),
    Cybersecurity(CybersecurityList),
    Dependencies(DependenciesList),
    Deployment(DeploymentList),
    Documentation(DocumentationList),
    Fun(FunList),
    Impermanent(ImpermanentList),
    Improvement(ImprovementList),
    Infrastructure(InfrastructureList),
    Metadata(MetadataList),
    Persistence(PersistenceList),
    Presentation(PresentationList),
    Testing(TestingList),
}

const EMOJIS: [&'static str; 15] = [
    "Accessibility - Accessibility and / or discoverability of the project / repository",
    "Architecture - Modify the design, shape, or architecture of the project / repository",
    "Core - Core to the central purpose of the project / repository",
    "Cybersecurity - Have cybersecurity implications",
    "Dependencies - Changing dependencies",
    "Deployment - Deployment of the project / repository",
    "Documentation - Documentation of Internal implementation of the project / repository",
    "Fun - Just for fun :)",
    "Impermanent - May not be permanent",
    "Improvement - Incremental update to what already exists in the project / repository",
    "Infrastructure - Building and developing of the project / repository",
    "Metadata - Project / repository metadata",
    "Persistence - How the project / repository persists information",
    "Presentation - Presentation of the project / repo to the user",
    "Testing - Test or mock used in tests",
];

pub struct MainList {
    state: ListState,
}

impl MainList {
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

    pub fn select(&mut self) -> Option<ScreenMode> {
        if let Some(selected_index) = self.state.selected() 
            && let Some(group) = EMOJIS.get(selected_index)
            && let Some((group, _)) = group.split_once(" ") {
            match group {
                "Accessibility" => Some(ScreenMode::Accessibility(AccessibilityList::new())),
                "Architecture" => Some(ScreenMode::Architecture(ArchitectureList::new())),
                "Core" => Some(ScreenMode::Core(CoreList::new())),
                "Cybersecurity" => Some(ScreenMode::Cybersecurity(CybersecurityList::new())),
                "Dependencies" => Some(ScreenMode::Dependencies(DependenciesList::new())),
                "Deployment" => Some(ScreenMode::Deployment(DeploymentList::new())),
                "Documentation" => Some(ScreenMode::Documentation(DocumentationList::new())),
                "Fun" => Some(ScreenMode::Fun(FunList::new())),
                "Impermanent" => Some(ScreenMode::Impermanent(ImpermanentList::new())),
                "Improvement" => Some(ScreenMode::Improvement(ImprovementList::new())),
                "Infrastructure" => Some(ScreenMode::Infrastructure(InfrastructureList::new())),
                "Metadata" => Some(ScreenMode::Metadata(MetadataList::new())),
                "Persistence" => Some(ScreenMode::Persistence(PersistenceList::new())),
                "Presentation" => Some(ScreenMode::Presentation(PresentationList::new())),
                "Testing" => Some(ScreenMode::Testing(TestingList::new())),
                _ => None
            }
        } else {
            None
        }
    }
}

impl Widget for &mut MainList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("Choose your Emoji").centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let list = List::new(EMOJIS.map(|commit_type| format!("{}", commit_type)))
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

pub fn copy_to_clipboard(contents: String) {
    std::process::Command::new("wl-copy").arg(contents).spawn().expect("Expected wl-copy process to run without errors");
}
