use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use anyhow::{Ok, Result};
use crossterm::event::{KeyEventKind, KeyModifiers};
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

pub fn ui(mut input: Vec<Author>) -> Result<Option<Vec<Author>>> {
    let (mut theight, mut twith) = terminal::size()?;
    setup_ui()?;
    let mut stdout = stdout();

    let mut search = String::new();
    let bind_authors = input.clone();
    let mut filtered_authors = filter_authors(&bind_authors, search.to_string());
    let mut selected: usize = 0;
    let mut saved = false;

    'ui: loop {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Key(m) => {
                    if m.kind == KeyEventKind::Press {
                        match m.code {
                            KeyCode::Esc => break 'ui,
                            KeyCode::Enter => {
                                saved = true;
                                break 'ui
                            },
                            KeyCode::Char('r') => {
                                if m.modifiers.contains(KeyModifiers::CONTROL) {
                                    input = input
                                        .into_iter()
                                        .map(|mut a| {
                                            a.staged = false;
                                            a
                                        })
                                        .collect();
                                    filtered_authors = filter_authors(&input, search.to_string());
                                }
                            }
                            KeyCode::Char(' ') => {
                                if let Some(s) = filtered_authors.get(selected) {
                                    input = input
                                        .clone()
                                        .into_iter()
                                        .map(|mut a| {
                                            if a.name == s.name && a.email == s.email {
                                                a.staged = !a.staged;
                                            }
                                            a
                                        })
                                        .collect();
                                    filtered_authors = filter_authors(&input, search.to_string());
                                }
                            }
                            KeyCode::Char(c) => {
                                search.push(c);
                                filtered_authors = filter_authors(&input, search.to_string());
                                let len = filtered_authors.len();
                                if len > 1 {
                                    let last = len - 1;
                                    if selected > last {
                                        selected = last;
                                    }
                                }
                                if len == 0 {
                                    selected = 0;
                                }
                            }
                            KeyCode::Backspace => {
                                search.pop();
                                filtered_authors = filter_authors(&input, search.to_string());
                            }
                            KeyCode::Up => {
                                if selected != usize::MIN {
                                    selected = selected.saturating_sub(1);
                                }
                            }
                            KeyCode::Down => {
                                if selected != usize::MAX && selected < filtered_authors.len() - 1 {
                                    selected = selected.saturating_add(1);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Event::Resize(w, h) => {
                    twith = w;
                    theight = h;
                }
                _ => (),
            }
        }

        let lines = render_canvas(
            &theight,
            &twith,
            &search,
            &selected,
            &filtered_authors,
        );
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        for line in lines.iter() {
            stdout.queue(cursor::MoveToNextLine(1))?;
            stdout.write_all(line.as_bytes())?;
        }
        stdout.queue(cursor::MoveTo(0, theight))?;
        stdout.write_all("Usage: <Esc>: Close <Enter>: Edit and close, <space>: Stage author, arrow up/down: Move hover, Ctrl-r: Remove all checkmarks".as_bytes())?;
        stdout.flush()?;

        thread::sleep(Duration::from_millis(10));
    }
    teardown_ui()?;
    if saved {
      Ok(Some(input.into_iter().filter(|a| a.staged).collect()))
    } else {
      Ok(None)
    }
}

fn filter_authors(authors: &Vec<Author>, search: String) -> Vec<&Author> {
    let search = search.to_lowercase();
    authors
        .into_iter()
        .filter(|c| c.name.to_lowercase().contains(&search))
        .collect()
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
