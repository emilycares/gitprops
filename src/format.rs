use crate::config::Author;


pub fn format_commit_message<'a>(message: &'a str, authors: Vec<Author>) -> String {
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
    use crate::{config::Author, format::{format_commit_message, parse_authors}};

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
        let result = format_commit_message(message, vec![Author::new("me", "doNotLook")]);
        assert_eq!(result, "supper change\nCo-authored-by: me <doNotLook>");
    }

    #[test]
    fn format_commit_message_add_another_one() {
        let message = "supper change\nCo-authored-by: me <doNotLook>";
        let result = format_commit_message(
            message,
            vec![
                Author::new("me", "doNotLook"),
                Author::new("notMe", "neveeSeen"),
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
                Author::new("me", "doNotLook"),
                Author::new("notMe", "neveeSeen"),
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
        let result = format_commit_message(message, vec![Author::new("me", "doNotLook")]);
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
