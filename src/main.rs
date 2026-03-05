// pingray

use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use futures::StreamExt;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::stdout;
use tokio::sync::mpsc;

mod app;
mod config;
mod probe;
mod targets;
mod ui;

type Mode = (&'static str, fn() -> config::Config, &'static targets::TargetGroup);

const MODES: &[Mode] = &[
    ("dns",    config::Config::default_root_dns,   &targets::ROOT_DNS),
    ("cctld",  config::Config::default_cctld,      &targets::CCTLD_DNS),
    ("ntp",    config::Config::default_global_ntp,  &targets::GLOBAL_NTP),
    ("ixp",    config::Config::default_ixp,         &targets::IXP_SERVERS),
    ("ntp-ca", config::Config::default_canada,      &targets::CANADIAN_NTP),
];

#[tokio::main]
async fn main() -> Result<()> {
    // terminal setup
    enable_raw_mode()?;
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal).await;

    // teardown — always runs
    disable_raw_mode()?;
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;

    result
}

async fn run(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let arg = std::env::args().nth(1).unwrap_or_default();
    let mut mode_idx = MODES.iter()
        .position(|(name, _, _)| *name == arg)
        .unwrap_or(0);

    let (mut cfg, mut group) = {
        let (_, mk_cfg, g) = &MODES[mode_idx];
        (mk_cfg(), *g)
    };
    let mut app = app::App::new(group);

    let (tx, mut rx) = mpsc::unbounded_channel();
    probe::spawn_probe_loop(group.targets, tx, cfg.probe_timeout_ms, cfg.probe_interval_secs);

    let mut events = EventStream::new();

    // render initial frame
    terminal.draw(|f| ui::draw(f, &app, &cfg))?;

    loop {
        tokio::select! {
            Some(results) = rx.recv() => {
                app.update(results);
                terminal.draw(|f| ui::draw(f, &app, &cfg))?;
            }
            Some(Ok(event)) = events.next() => {
                if let Event::Key(key) = event {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Tab => {
                            mode_idx = (mode_idx + 1) % MODES.len();
                            let (_, mk_cfg, g) = &MODES[mode_idx];
                            cfg = mk_cfg();
                            group = *g;
                            app = app::App::new(group);

                            // new channel kills old probe loop (send fails → breaks)
                            let (new_tx, new_rx) = mpsc::unbounded_channel();
                            rx = new_rx;
                            probe::spawn_probe_loop(group.targets, new_tx, cfg.probe_timeout_ms, cfg.probe_interval_secs);
                        }
                        _ => {}
                    }
                }
                terminal.draw(|f| ui::draw(f, &app, &cfg))?;
            }
        }

        if !app.running {
            break;
        }
    }

    Ok(())
}
