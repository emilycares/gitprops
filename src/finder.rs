use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use anyhow::{Ok, Result};
use crossterm::{
    cursor::{self, Hide, Show},
    event::{poll, Event},
    event::{read, KeyCode},
    style::Stylize,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};

use crate::config::Author;

pub fn ui() -> Result<()> {
    let (mut theight, mut twith) = terminal::size()?;
    setup_ui()?;
    let mut stdout = stdout();

    let mut search = String::new();
    let mut authors = vec![
        Author::new("Bob", "bob@gmail.com"),
        Author::new("John", "john@gmail.com"),
        Author::new("Alice", "alice@gmail.com"),
    ];
    let mut filtered_authors = filter_authors(&authors, &search);
    let mut selected: usize = 0;

    'ui: loop {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Key(m) => match m.code {
                    KeyCode::Esc => break 'ui,
                    KeyCode::Char(c) => {
                        search.push(c);
                        filtered_authors = filter_authors(&authors, &search);
                        let len = filtered_authors.len();
                        if len > 1 {
                            let last = len - 1;
                            if selected > last {
                                selected = last;
                            }
                        }
                    }
                    KeyCode::Up => {
                        if selected != usize::MIN {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected != usize::MAX {
                            selected += 1;
                        }
                    }
                    KeyCode::Backspace => {
                        search.pop();
                        filtered_authors = filter_authors(&authors, &search);
                    }
                    _ => (),
                },
                Event::Resize(w, h) => {
                    twith = w;
                    theight = h;
                }
                _ => (),
            }
        }

        let lines = render_canvas(&theight, &twith, &search, &selected, &filtered_authors);
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        for (i, line) in lines.iter().enumerate() {
            stdout.queue(cursor::MoveTo(0, i.try_into().unwrap_or_default()))?;
            stdout.write(line.as_bytes())?;
        }
        stdout.flush()?;

        thread::sleep(Duration::from_millis(10));
    }
    teardown_ui()?;

    Ok(())
}

fn filter_authors<'a>(authors: &'a Vec<Author>, search: &'a str) -> Vec<&'a Author> {
    return authors
        .iter()
        .filter(|c| c.name.to_lowercase().contains(search))
        .collect();
}

fn render_canvas(
    _theight: &u16,
    _twith: &u16,
    search: &str,
    selected: &usize,
    authors: &Vec<&Author>,
) -> Vec<String> {
    let mut out = vec![format!("Search: {search}")];
    out.extend(render_authors(authors, selected));
    out
}

fn render_authors(authors: &Vec<&Author>, selected: &usize) -> Vec<String> {
    authors
        .iter()
        .enumerate()
        .map(|(i, author)| {
            let mut line = author.to_string();
            if &i == selected {
                line = line.green().to_string();
            }
            line
        })
        .collect()
}

fn setup_ui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    Ok(())
}
fn teardown_ui() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
