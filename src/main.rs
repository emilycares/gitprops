use git2::Repository;
fn main() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let Ok(head) = repo.head() else {
        println!("Unable to get head");
        return;
    };
    let Ok(commit) = head.peel_to_commit() else {
        println!("Unable to get last commit");
        return;
    };
    let existing_message = commit.message();
    println!("existing_message {existing_message:#?}");
    let message = get_new_commit_message(existing_message);
    if message.is_some() {
        match commit.amend(None, None, None, None, message, None) {
            Ok(_) => println!("Gave props"),
            Err(_) => println!("Unable to edit commit"),
        }
    }
}

fn get_new_commit_message(existing_message: Option<&str>) -> Option<&str> {
    None
}
