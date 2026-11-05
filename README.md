# GooseMusic TinySkia

A GooseMusic v1.5.1 fork using Iced's TinySkia renderer. The point of this branch was to get TinySkia working without changing the rest of the app.

The renderer changes are on [tiny-skia-v1.5.1](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/tiny-skia-v1.5.1).

## Changes

- Based on GooseMusic v1.5.1, upstream commit `e1d27d8e`.
- Uses Iced `0.14`.
- Enables the Iced `tiny-skia` feature.
- Removes the explicit Iced `wgpu` feature.
- Keeps `image`, `svg`, `tokio`, `canvas`, `advanced`, `crisp`, `web-colors`, `thread-pool`, `x11`, and `wayland`.
- Updates `Cargo.lock`.

The migration commit changes `Cargo.toml` and `Cargo.lock`. The lockfile diff is 14 insertions and 332 deletions. WGPU-related packages can still appear in the lockfile. We removed the explicit Iced WGPU feature, not necessarily every WGPU-related dependency.

## Memory usage

We measured process memory using `/proc/<pid>/smaps_rollup` and process status information.

| Build | RSS | PSS |
| --- | ---: | ---: |
| Original, without TinySkia | 126,816 kB (123.8 MiB) | 85,354 kB (83.4 MiB) |
| TinySkia | 51,980 kB (50.8 MiB) | 40,444 kB (39.5 MiB) |

A later TinySkia reading during playback was 56,520 kB RSS (55.2 MiB) and 45,563 kB PSS (44.5 MiB).

These are readings from our tests, not a formal benchmark. The original build was measured idle, while the first TinySkia reading was taken during playback, so the workloads were not identical. The numbers are useful as observed memory usage, but should not be treated as a controlled head-to-head result.

## Build and testing

The optimized release build completed in about 2 minutes 30 seconds on the test machine.

We tested:
- Audio playback
- Album artwork
- Mosaic rendering
- SVG rendering

All four worked.

Test machine:
- CachyOS
- Wayland with Hyprland
- Intel Core i5-12400T
- Intel UHD Graphics 730
- 16 GB RAM

This was tested on one machine. We did not run a cross-platform test matrix or a controlled FPS benchmark.

## Cross-platform compatibility

We are roughly **90% confident** that cross-platform compatibility should remain intact. This is an engineering estimate based on the scope of the change, not a measured compatibility rate.

The change was limited to the renderer configuration: we enabled Iced's TinySkia feature and removed the explicit Iced `wgpu` feature. We did not intentionally change the audio, provider, playback, media-control, file-handling, or platform-specific application code. The existing X11 and Wayland features were kept.

In other words, we changed how the interface is drawn, not the parts responsible for playing music or connecting to providers. The build and runtime checks passed on our CachyOS test machine, but Windows, macOS, and other Linux environments have not been tested after this change. So cross-platform support is expected to remain, but is not yet confirmed on every platform.

## Known issue

Very long track titles can sometimes overlap the next row in the upright track list. It is a small visual issue and was left as-is.

## Build

You need the Rust toolchain.

```bash
git clone --branch tiny-skia-v1.5.1 https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr.git
cd music_plr
cargo build --release
```

The binary is in `target/release/`.

## Links

- [TinySkia branch](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/tree/tiny-skia-v1.5.1)
- [Migration commit 614e954](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/commit/614e9543932a94b0ff5a1498bd219fb095c2604e)
- [Original GooseMusic project](https://github.com/GooseOb/music_plr)
