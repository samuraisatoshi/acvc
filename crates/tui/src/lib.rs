//! Driving adapter: terminal viewer.
#![forbid(unsafe_code)]

mod model;
mod ui;

use std::io::{self, stdout};

use acvc_app::App;
use acvc_domain::{Error, Result};
use model::{Command, Model};
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::{CrosstermBackend, Terminal};

pub use model::{Command as TuiCommand, Model as TuiModel};

struct Restorer;

impl Drop for Restorer {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

pub fn run(app: &App) -> Result<()> {
    enable_raw_mode().map_err(io_err)?;
    execute!(stdout(), EnterAlternateScreen).map_err(io_err)?;
    let _restorer = Restorer;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).map_err(io_err)?;
    let mut model = Model::load(app)?;
    loop {
        terminal
            .draw(|frame| ui::draw(frame, &model))
            .map_err(io_err)?;
        if let Event::Key(key) = event::read().map_err(io_err)? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            let command = model.on_key(key.code);
            if apply(app, &mut model, command)? {
                break;
            }
        }
    }
    Ok(())
}

fn apply(app: &App, model: &mut Model, command: Command) -> Result<bool> {
    match command {
        Command::Quit => Ok(true),
        Command::None => Ok(false),
        Command::Reload => {
            model.reload(app)?;
            Ok(false)
        }
        Command::LoadFrontier(campaign) => {
            model.frontier = app.frontier(&campaign)?;
            model.frontier_idx = 0;
            model.screen = model::Screen::Frontier;
            Ok(false)
        }
        Command::LoadDetail(id) => {
            model.detail = Some(app.why(&id)?);
            model.lineage_idx = 0;
            model.screen = model::Screen::Detail;
            Ok(false)
        }
        Command::LoadCompare(left, right) => {
            let report = app.compare(&left, &right)?;
            model.diff_paths = report
                .diff
                .modified
                .iter()
                .chain(report.diff.added.iter())
                .chain(report.diff.removed.iter())
                .cloned()
                .collect();
            model.diff_idx = 0;
            model.compare = Some(report);
            model.file_text.clear();
            model.screen = model::Screen::Diff;
            Ok(false)
        }
        Command::LoadFile { left, right, path } => {
            model.file_text = app.file_diff(&left, &right, &path)?;
            Ok(false)
        }
        Command::Recall { campaign, query } => {
            model.recall_hits = app.recall(&campaign, &query)?;
            model.typing = false;
            model.screen = model::Screen::Recall;
            Ok(false)
        }
    }
}

fn io_err(err: io::Error) -> Error {
    Error::new(err.to_string()).ctx("op", "terminal")
}
