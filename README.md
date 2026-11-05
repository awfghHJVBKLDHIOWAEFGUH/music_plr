# GooseMusic — TinySkia fork

An experimental fork of [GooseMusic](https://github.com/GooseOb/music_plr) v1.5.1 that switches the Iced renderer configuration to **TinySkia** and removes the application's explicit Iced `wgpu` feature.

This fork keeps GooseMusic's existing application, audio playback, providers, media controls, and interface. The renderer/dependency experiment is isolated on the [`tiny-skia-v1.5.1` branch](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/tiny-skia-v1.5.1).

> **Status:** Builds successfully and has passed the runtime checks listed below. This is an experimental configuration, not a claim of measured performance gains or a replacement for the upstream project.

## What changed

- **Base version:** GooseMusic v1.5.1, upstream source commit `e1d27d8e`.
- **Iced:** `0.14`.
- **Renderer:** enabled Iced's `tiny-skia` feature.
- **Feature change:** removed the explicit `wgpu` feature from the Iced dependency.
- **Resolved TinySkia crates:** `iced_tiny_skia 0.14.1` and `tiny-skia 0.11.4`.
- **Other Iced features retained:** `image`, `svg`, `tokio`, `canvas`, `advanced`, `crisp`, `web-colors`, `thread-pool`, `x11`, and `wayland`.

The migration commit changes only `Cargo.toml` and `Cargo.lock`; application UI and audio source files were not changed as part of the renderer migration.

The lockfile diff contains **14 insertions and 332 deletions**. WGPU-related package records may still appear in the dependency graph; removing the explicit Iced `wgpu` feature does **not** mean every WGPU-named crate has been eliminated.

## Build and validation

A release build completed successfully:

- **Build profile:** optimized release
- **Recorded build time:** **2 minutes 30 seconds**

Runtime checks completed successfully for:

- Audio playback
- Album artwork
- Mosaic rendering
- SVG rendering

### Test environment

| Component | Test system |
|---|---|
| OS | CachyOS |
| Display session | Wayland / Hyprland |
| CPU | Intel Core i5-12400T |
| Graphics | Intel UHD Graphics 730 |
| Memory | 16 GB RAM |

These are local test results from one system, not a cross-platform test matrix. No controlled comparison against the WGPU renderer was performed, so **no FPS, memory-usage, or speed improvement is claimed**.

## Known limitation

Very long track titles can occasionally crowd or overlap adjacent rows in the upright track list. It is a minor presentation issue; the attempted layout adjustment was not included in the migration commit.

## Build it

With the Rust toolchain installed:

```bash
git clone --branch tiny-skia-v1.5.1 https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr.git
cd music_plr
cargo build --release
```

The resulting binary is produced by Cargo in `target/release/`.

## Branches and commits

- **TinySkia implementation:** [`tiny-skia-v1.5.1`](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/tiny-skia-v1.5.1)
- **Migration commit:** [`614e954`](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/commit/614e9543932a94b0ff5a1498bd219fb095c2604e)
- **Based on upstream:** [GooseOb/music_plr](https://github.com/GooseOb/music_plr), v1.5.1

The default `master` branch contains this fork-specific README. The actual renderer/dependency changes remain on `tiny-skia-v1.5.1`.
