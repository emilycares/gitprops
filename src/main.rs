mod config;
mod finder;
use clap::Parser;
use config::Args;
use git2::Repository;

fn main() {
    let args = Args::parse();
    if args.ui {
        finder::ui().unwrap();
    }
    if false {
        let Ok(repo) = Repository::open("./") else {
            println!("failed to open repo");
            return;
        };
        let Ok(head) = repo.head() else {
            println!("Unable to get head");
            return;
        };
        let Ok(commit) = head.peel_to_commit() else {
            println!("Unable to get last commit");
            return;
        };
        set_commit_message(commit, &args);
    }
}

fn set_commit_message<'a>(commit: git2::Commit<'a>, args: &Args) {
    let message = match commit.message() {
        Some(msg) => msg,
        None => "",
    };
    let message = format_commit_message(message, &args);
    match commit.amend(Some("HEAD"), None, None, None, Some(&message), None) {
        Ok(_) => println!("Gave props"),
        Err(_) => println!("Unable to edit commit"),
    }
}

fn format_commit_message<'a>(existing_message: &'a str, args: &Args) -> String {
    format!(
        "{existing_message}\n\nCo-authored-by: {} <{}>",
        args.name, args.email
    )
}
