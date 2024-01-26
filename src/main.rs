mod config;
mod finder;
mod format;
use clap::Parser;
use config::{Args, Author};
use git2::Repository;

fn main() {
    let args = Args::parse();
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
    let message = match commit.message() {
        Some(msg) => msg,
        None => "",
    };
    if args.ui {
        let authors = vec![
            Author::new("Bob", "bob@gmail.com"),
            Author::new("John", "john@gmail.com"),
            Author::new("Alice", "alice@gmail.com"),
        ];
        let existing_authors = format::parse_authors(message);
        let authors = mark_present(authors, existing_authors);
        let a = finder::ui(authors);
        if let Ok(a) = a {
            let message = format::format_commit_message(message, a);
            println!("{}", message);
            set_commit_message(commit, message);
        }
    } else {
        let message = format::format_commit_message(
            message,
            vec![Author {
                name: args.name,
                email: args.email,
                staged: false,
            }],
        );
        println!("{}", message);
        set_commit_message(commit, message);
    }
}

fn mark_present(authors: Vec<Author>, existing_authors: Vec<&str>) -> Vec<Author> {
    return authors
        .into_iter()
        .map(|mut a| {
            let email = a.email.as_str();
            a.staged = existing_authors.contains(&email);
            a
        })
        .collect();
}

fn set_commit_message<'a>(commit: git2::Commit<'a>, message: String) {
    match commit.amend(Some("HEAD"), None, None, None, Some(&message), None) {
        Ok(_) => (),
        Err(_) => println!("Unable to edit commit"),
    }
}
