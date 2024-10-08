use conventional::CommitMessage;
use std::fs;
use std::path::Path;

mod conventional {
    use regex::Regex;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub struct CommitMessage {
        message: String,
        emoji: char,
    }

    impl CommitMessage {
        pub fn try_new(message: String) -> Result<CommitMessage, String> {
            let conventional_commit_re =
                Regex::new(r#"^(\w+){1}(\([\w\-\.]+\))?(!)?: (\p{Emoji})?([\w ])+([\s\S]*)"#)
                    .unwrap();
            if !conventional_commit_re.is_match(&message) {
                return Err(String::from("expected conventional commit message"));
            }

            let emoji = match &message {
                msg if msg.starts_with("build") => '📦',
                msg if msg.starts_with("chore") => '🔧',
                msg if msg.starts_with("ci") => '👷',
                msg if msg.starts_with("docs") => '📚',
                msg if msg.starts_with("feat") => '✨',
                msg if msg.starts_with("fix") => '🐛',
                msg if msg.starts_with("perf") => '⚡',
                msg if msg.starts_with("refactor") => '🔄',
                msg if msg.starts_with("revert") => '⏪',
                msg if msg.starts_with("style") => '🎨',
                msg if msg.starts_with("test") => '🧪',
                _ => return Err(String::from("expected valid conventional commit")),
            };

            Ok(CommitMessage { message, emoji })
        }

        pub fn insert_emoji(self) -> Result<EmojiCommitMessage, String> {
            let Some(insertion_idx) = self.message.find(':') else {
                panic!("expected a conventional commit message");
            };

            let emoji_re = Regex::new(
                r#"^(\w+){1}(\([\w\-\.]+\))?(!)?: (\p{Emoji})([\w ])+([\s\S]*)\p{Emoji}"#,
            )
            .unwrap();
            if emoji_re.is_match(&self.message) {
                // Already contains an emoji, probably running git commit --amend.
                // Return the message as it
                return Ok(EmojiCommitMessage(self.message));
            }

            let mut ret_val = String::from(self.message);
            let insertion = String::from(" ") + self.emoji.to_string().as_str();
            ret_val.insert_str(insertion_idx + 1, insertion.as_str());

            Ok(EmojiCommitMessage(ret_val))
        }
    }

    impl fmt::Display for CommitMessage {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct EmojiCommitMessage(String);

    impl fmt::Display for EmojiCommitMessage {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

fn main() {
    if !Path::new("./.git/").try_exists().unwrap() {
        panic!("must run this hook in a git repository");
    }

    if !Path::new("./.git/COMMIT_EDITMSG").try_exists().unwrap() {
        panic!("must run this hook in the commit-msg stage");
    }

    let commit_msg = fs::read("./.git/COMMIT_EDITMSG").expect("we can read from the file");
    let commit_msg = String::from_utf8(commit_msg).expect("no invalid UTF-8 in commit message");

    println!("commit_msg: {}", commit_msg);

    // Source: https://github.com/folke/devmoji?tab=readme-ov-file#default-devmoji-reference
    let conventional_commit_msg = CommitMessage::try_new(commit_msg).unwrap();
    let modified_commit_msg = conventional_commit_msg.insert_emoji().unwrap();

    fs::write(
        "./.git/COMMIT_EDITMSG",
        modified_commit_msg.to_string().as_bytes(),
    )
    .expect("we can write to the file");
}

#[cfg(test)]
mod tests {
    use super::conventional::*;

    #[test]
    fn test_valid_conventional_commit() {
        // Arrange
        let commit_msgs = [
            "build(foo): bar baz",
            "build: bar baz",
            "chore(foo): bar baz",
            "chore: bar baz",
            "ci(foo): bar baz",
            "ci: bar baz",
            "docs(foo): bar baz",
            "docs: bar baz",
            "feat(foo): bar baz",
            "feat: bar baz",
            "fix(foo): bar baz",
            "fix: bar baz",
            "perf(foo): bar baz",
            "perf: bar baz",
            "refactor(foo): bar baz",
            "refactor: bar baz",
            "revert(foo): bar baz",
            "revert: bar baz",
            "style(foo): bar baz",
            "style: bar baz",
            "test(foo): bar baz",
            "test: bar baz",
        ];

        for commit_msg in commit_msgs {
            // Act
            let conventional_commit_msg = CommitMessage::try_new(String::from(commit_msg));

            // Assert
            assert_eq!(
                conventional_commit_msg.err(),
                None,
                "Failed to parse conventional commit from \"{commit_msg}\""
            );
        }
    }

    #[test]
    fn test_invalid_conventional_commit() {
        // Arrange
        let commit_msgs = [
            "foo bar baz",
            "asdf(foo): bar baz",
            "asdf: bar baz",
            "feat",
            "feat:",
            "feat: ",
            "feat(foo):",
            "feat(food): ",
        ];

        for commit_msg in commit_msgs {
            // Act
            let conventional_commit_msg = CommitMessage::try_new(String::from(commit_msg));

            // Assert
            assert_eq!(
                conventional_commit_msg.ok(),
                None,
                "Should have failed to parse a conventional commit from \"{commit_msg}\""
            );
        }
    }
}
