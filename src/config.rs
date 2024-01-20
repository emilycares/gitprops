use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "false")]
    pub ui: bool,
    #[arg(short, long, default_value = "", required_if_eq("ui", "false"))]
    pub name: String,
    #[arg(short, long, default_value = "", required_if_eq("ui", "false"))]
    pub email: String,
}

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

impl Author {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            staged: false,
        }
    }
}
