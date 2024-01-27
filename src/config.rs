use clap::Parser;
use serde::{Deserialize, Serialize};

/// This is a tool to add "Co-authored-by" the last commit. In a TUI. That includes a search.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Print config location
    #[arg(long, default_value = "false")]
    pub print_config_location: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub authors: Vec<StorageAuthor>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            authors: vec![StorageAuthor {
                name: "Nice Person".to_owned(),
                email: "nice@email".to_owned(),
            }],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StorageAuthor {
    pub name: String,
    pub email: String,
}

impl From<StorageAuthor> for Author {
    fn from(value: StorageAuthor) -> Self {
        Self {
            name: value.name,
            email: value.email,
            staged: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub staged: bool,
}

impl ToString for Author {
    fn to_string(&self) -> String {
        let staged = match self.staged {
            true => "X",
            false => " ",
        };
        format!("[{}] {} {}", staged, self.name, self.email)
    }
}
