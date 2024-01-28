mod config;
mod finder;
mod format;
use clap::Parser;
use config::{Args, Author, Storage};
use git2::Repository;

#[tokio::main()]
async fn main() {
    let args = Args::parse();
    let storage_location = &quickcfg::get_location("gitprops")
        .await
        .expect("Unable to get storage dir");
    let storage: Storage = quickcfg::load(storage_location).await;

    if args.print_config_location {
        println!("{}", storage_location);
        return;
    }
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
    let message = commit.message().unwrap_or("");
    let authors: Vec<Author> = storage.authors.into_iter().map(|a| a.into()).collect();

    let existing_authors = format::parse_authors(message);
    let authors = mark_present(authors, existing_authors);
    match finder::ui(authors) {
        Ok(Some(authors)) => {
            let message = format::format_commit_message(message, authors);
            println!("{}", message);
            set_commit_message(commit, message);
        }
        Ok(None) => {
            println!("Aborted. No commit message was changed")
        }
        Err(e) => eprintln!("Got an error in the ui: {:?}", e),
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

fn set_commit_message(commit: git2::Commit<'_>, message: String) {
    match commit.amend(Some("HEAD"), None, None, None, Some(&message), None) {
        Ok(_) => (),
        Err(_) => println!("Unable to edit commit"),
    }
}
