# Last Process

Minimal 2D platformer prototype built in Rust with [Bevy](https://bevyengine.org/) 0.20.0.

## Gameplay

The current build is a small physics sandbox: move a single player capsule across a platform layout and jump between elevated surfaces. There are no enemies, collectibles, level transitions, UI, or art/audio assets in use yet.

## Controls

| Action | Keys |
| --- | --- |
| Move left | `A` / `←` |
| Move right | `D` / `→` |
| Jump | `Space` / `W` / `↑` |

Jump input is only accepted while the player is grounded.

## Running

Requires a recent stable [Rust](https://rustup.rs/) toolchain.

```sh
cargo run            # debug build
cargo run --release  # optimized build
```

## Architecture

The entire game logic lives in `src/main.rs`.

- `Player` component: a 40×64 sprite with `velocity: Vec2` and `grounded: bool`.
- `Platform` component: rectangular collision volumes with a `size: Vec2`.
- `setup`: spawns the camera, player, floor, and two elevated platforms.
- `player_movement`: runs each frame and handles:
  - horizontal input and acceleration
  - jump state checks
  - gravity (`1800.0`)
  - custom AABB collision against platform edges and tops
  - landing detection when falling intersects a platform surface

Physics is manually resolved rather than using a full rigid-body or physics engine; the player is treated as a rectangular body moving through a static platform set.

## Project layout

```text
src/main.rs   Game scene, components, input handling, custom collision logic
assets/      Reserved for future sprites, audio, fonts, and levels
tests/       No integration tests yet
Cargo.toml    Rust package manifest with Bevy 0.20.0
```

## Status

Early prototype. All visuals are flat colored rectangles, and the project is still a single-scene physics test rather than a complete game.
