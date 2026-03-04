# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**pingray** is a Rust TUI app that shows a live map with ping rays going out to network targets. It visualizes latency to servers on a North America canvas map with color-coded rays (green < 30ms, yellow < 80ms, red). Currently targets Canadian NTP servers; DNS resolvers planned next.

## Build & Run

```bash
cargo build          # compile
cargo run            # build and run (launches TUI, press q to quit)
cargo test           # run tests (none yet)
cargo clippy         # lint
```

Requires Rust edition 2024 (nightly or recent stable).

## Architecture

- **src/main.rs** — Entry point. Sets up crossterm terminal (raw mode, alternate screen), spawns probe loop, runs event loop with `tokio::select!` on keyboard events and probe results, renders TUI.
- **src/app.rs** — `App` state struct. Holds target group, latest `ProbeResult` per target, per-target latency history, scrolling log. Provides stats (median, jitter, loss count).
- **src/ui.rs** — TUI rendering. Three panels: Canvas map (ratatui `Map` + colored `Line` rays from user to targets + `Points`), stats sidebar (per-host latency list, median/jitter/loss), and scrolling log.
- **src/probe.rs** — Async NTP probing. Spawns concurrent queries per target with per-host timeout, measures round-trip latency, sends `Vec<ProbeResult>` via mpsc channel every 3 seconds.
- **src/targets.rs** — Target definitions. `Target` struct (name, host, lat, lon), `TargetGroup`. Currently defines 13 Canadian stratum-1 NTP servers with geographic coordinates.

## Key Dependencies

- **ratatui** — TUI framework (Canvas widget for map, layout, styling)
- **crossterm** (with `event-stream` feature) — terminal backend + async keyboard events
- **rsntp** (with `chrono` feature) — async SNTP client for NTP probing
- **tokio** — async runtime
- **futures** — `StreamExt` for crossterm `EventStream`
- **anyhow** — error handling
