use std::env::args;

fn main() {
    let args = args();

    let Some(commit_msg) = args.into_iter().next() else {
        panic!("expected commit message as a command-line argument!");
    };

    match commit_msg {
        _ => panic!("expected a conventional commit message"),
    }
}
