mod config;
mod finder;
use clap::Parser;
use config::{Args, Author};
use git2::Repository;

fn main() {
    let args = Args::parse();
    let Some((commit, message)) = get_commit() else {
        return;
    };
    if args.ui {
        let authors = vec![
            Author::new("Bob", "bob@gmail.com"),
            Author::new("John", "john@gmail.com"),
            Author::new("Alice", "alice@gmail.com"),
        ];
        let existing_authors = parse_authors(message);
        let authors = mark_present(authors, existing_authors);
        finder::ui(authors, |a| {
            if let Some(a) = a {
                let message = format_commit_message(message, vec![a]);
                set_commit_message(commit, message);
            }
        })
        .unwrap();
    } else {
        let message = format_commit_message(
            message,
            vec![&Author {
                name: args.name,
                email: args.email,
                staged: false,
            }],
        );
        set_commit_message(commit, message);
    }
}

fn get_commit<'a>() -> Option<(git2::Commit<'a>, &'a str)> {
    let Ok(repo) = Repository::open("./") else {
        println!("failed to open repo");
        return None;
    };
    let Ok(head) = repo.head() else {
        println!("Unable to get head");
        return None;
    };
    let Ok(commit) = head.peel_to_commit() else {
        println!("Unable to get last commit");
        return None;
    };
    let message = match commit.message() {
        Some(msg) => msg,
        None => "",
    };
    Some((commit, message))
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
        Ok(_) => println!("Gave props"),
        Err(_) => println!("Unable to edit commit"),
    }
}

fn format_commit_message<'a>(message: &'a str, authors: Vec<&Author>) -> String {
    let mut msg: String = message
        .lines()
        .filter(|c| !c.contains("Co-authored-by"))
        .collect();
    for author in authors {
        msg.push_str(&format!(
            "\nCo-authored-by: {} <{}>",
            author.name, author.email
        ))
    }
    msg
}

pub fn parse_authors<'a>(message: &'a str) -> Vec<&'a str> {
    message
        .lines()
        .map(|c| c.trim())
        .filter(|c| c.starts_with("Co-authored-by"))
        .filter_map(|c| c.split_once("<"))
        .map(|c| c.1)
        .map(|c| &c[0..c.len() - 1])
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{config::Author, format_commit_message, parse_authors};

    #[test]
    fn parse_authors_basic() {
        let message = "Work\n\nHere is the message
            Co-authored-by: Great Person <otherEmail>
            Co-authored-by: Nice Person <email>";
        let result = parse_authors(message);
        assert_eq!(result, vec!["otherEmail", &"email"]);
    }

    #[test]
    fn format_commit_message_add() {
        let message = "supper change";
        let result = format_commit_message(message, vec![&Author::new("me", "doNotLook")]);
        assert_eq!(result, "supper change\nCo-authored-by: me <doNotLook>");
    }

    #[test]
    fn format_commit_message_add_another_one() {
        let message = "supper change\nCo-authored-by: me <doNotLook>";
        let result = format_commit_message(
            message,
            vec![
                &Author::new("me", "doNotLook"),
                &Author::new("notMe", "neveeSeen"),
            ],
        );
        assert_eq!(
            result,
            "supper change\nCo-authored-by: me <doNotLook>\nCo-authored-by: notMe <neveeSeen>"
        );
    }

    #[test]
    fn format_commit_message_add_more() {
        let message = "supper change";
        let result = format_commit_message(
            message,
            vec![
                &Author::new("me", "doNotLook"),
                &Author::new("notMe", "neveeSeen"),
            ],
        );
        assert_eq!(
            result,
            "supper change\nCo-authored-by: me <doNotLook>\nCo-authored-by: notMe <neveeSeen>"
        );
    }

    #[test]
    fn format_commit_message_remove_one() {
        let message =
            "supper change\nCo-authored-by: me <doNotLook>\nCo-authored-by: notMe <neveeSeen>";
        let result = format_commit_message(message, vec![&Author::new("me", "doNotLook")]);
        assert_eq!(result, "supper change\nCo-authored-by: me <doNotLook>");
    }
    #[test]
    fn format_commit_message_remove_all() {
        let message =
            "supper change\nCo-authored-by: me <doNotLook>\nCo-authored-by: notMe <neveeSeen>";
        let result = format_commit_message(message, vec![]);
        assert_eq!(result, "supper change");
    }
}
