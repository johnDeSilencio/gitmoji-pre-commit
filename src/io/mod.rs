use std::fs;

use serde::Deserialize;

const EMOJIS_TOML_PATH: &str = "emojis.toml";

#[derive(Debug, Deserialize)]
pub struct Groups {
    group: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ConventionalCommits {
    commit: Vec<ConventionalCommit>,
}

#[derive(Debug, Deserialize)]
pub struct ConventionalCommit {
    pub r#type: String,
    pub emoji: String,
    pub group: String,
    pub description: String,
}

pub fn read_emojis_toml() -> String {
    if let Ok(exists) = fs::exists(EMOJIS_TOML_PATH) && !exists {
        panic!("ERROR: Expected emojis.toml to exist");
    }

    let Ok(contents) = fs::read_to_string(EMOJIS_TOML_PATH) else {
        panic!("ERROR: Expecte to be ablet to read emojis.toml");
    };

    contents
}

pub fn parse_conventional_commits(contents: String) -> Vec<ConventionalCommit> {
    let conventional_commits: ConventionalCommits =
        toml::from_str(contents.as_str()).expect("Should have parsed correctly");

    conventional_commits.commit
}

pub fn parse_groups(contents: String) -> Vec<Group> {
    let groups: Groups = toml::from_str(contents.as_str()).expect("Should have parsed correctly");

    groups.group
}
