use std::env::args;

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
                Regex::new(r#"^(\w+){1}(\([\w\-\.]+\))?(!)?: ([\w ])+([\s\S]*)"#).unwrap();
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

            let emoji_re = Regex::new(r#"\p{Emoji}"#).unwrap();
            if emoji_re.is_match(&self.message) {
                return Err(String::from(
                    "expected conventional commit to not have emojis already in it",
                ));
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
    let args = args();

    let Some(commit_msg) = args.into_iter().next() else {
        panic!("expected commit message as a command-line argument!");
    };

    match commit_msg {
        _ => panic!("expected a conventional commit message"),
    }
}
