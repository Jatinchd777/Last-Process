# Last Process

A tiny 2D platformer prototype built with [Bevy](https://bevyengine.org/) 0.20.

## Gameplay

Run and jump across a small set of platforms. The current build is a physics
sandbox — there are no enemies, goals, or menus yet.

## Controls

| Action | Keys |
| --- | --- |
| Move left | `A` / `←` |
| Move right | `D` / `→` |
| Jump | `Space` / `W` / `↑` |

Jumping only works while grounded.

## Running

Requires a recent stable [Rust](https://rustup.rs/) toolchain.

```sh
cargo run            # debug build
cargo run --release  # faster
```

## How it works

Everything lives in `src/main.rs`:

- **Player** — a 40×64 solid-color sprite with a velocity and `grounded` flag.
- **Physics** — custom AABB collision with gravity (`1800`), move speed (`300`),
  and jump speed (`650`). Horizontal movement stops at platform sides; falling
  players land on platform tops.
- **Platforms** — a floor and two rising platforms, spawned from `setup`.
- **Systems** — `setup` runs once at startup; `player_movement` runs every frame.

## Project layout

```
src/main.rs     Game code and constants
assets/         Placeholder folders for sprites, audio, fonts, levels
tests/          Empty (integration tests TBD)
Cargo.toml      Bevy 0.20 dependency
```

## Status

Early prototype. Art and audio assets are not used yet; all visuals are
colored rectangles.
