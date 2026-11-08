# GooseOb's Music Player

![Screenshot](./docs/plr_screen.png)

A YouTube/SoundCloud/Bandcamp search music player with local playback, downloads, and OS media controls, built with [iced](https://iced.rs).


## v1.5.1 TinySkia fork

This branch is a focused renderer/dependency configuration experiment based on the official **GooseMusic v1.5.1** source tag. It keeps the existing application and its audio, provider, media-control, and UI code intact while enabling Iced's TinySkia renderer feature and dropping the app's explicit Iced `wgpu` feature.

### What changed

- **Base:** GooseMusic `v1.5.1`, based on upstream commit `e1d27d8e`.
- **Iced:** `0.14`.
- **TinySkia renderer crates resolved:** `iced_tiny_skia 0.14.1` and `tiny-skia 0.11.4`.
- **Cargo feature change:** removed the single explicit `wgpu` feature from the `iced` dependency; retained `tiny-skia` and the other 10 features: `image`, `svg`, `tokio`, `canvas`, `advanced`, `crisp`, `web-colors`, `thread-pool`, `x11`, and `wayland`.
- **Files changed for the migration:** `Cargo.toml` and `Cargo.lock`; no application UI or audio source files were changed in the migration commit.
- **Lockfile diff:** 14 insertions and 332 deletions (346 changed lines).

The feature change is specifically the removal of the app's explicit Iced `wgpu` feature. The committed lockfile still contains WGPU-related package records, so this README does not claim that every WGPU-named crate has been eliminated from dependency resolution.

### Build and runtime validation

- Optimized release build completed successfully.
- Recorded release-build time: **2 minutes 30 seconds** on the local test machine.
- Runtime checks passed for **audio playback, album artwork, mosaic rendering, and SVG rendering**.
- Validation environment: **CachyOS / Wayland / Hyprland**, Intel Core **i5-12400T**, Intel UHD Graphics **730**, and **16 GB RAM**.

These are local validation results, not a cross-platform test matrix. No controlled v1.5.1 performance benchmark was recorded, so no FPS, memory, or speed improvement is claimed here.

### Known limitation

Very long track titles can occasionally crowd/overlap adjacent rows in the upright track list. This is a minor presentation issue; the attempted layout adjustment was not included in the migration commit.

### Branch and commit

- Branch: `tiny-skia-v1.5.1`
- Migration commit: [`614e954`](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/commit/614e9543932a94b0ff5a1498bd219fb095c2604e)
- Fork: [awfghHJVBKLDHIOWAEFGUH/music_plr](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr)

This is a focused fork branch for testing the TinySkia configuration; it does not replace the upstream project's release or installation instructions.


## Features

- **Search** — YouTube Music (Songs / Videos / Artists / Albums / Playlists) via ytmusicapi with yt-dlp fallback, plus SoundCloud, Bandcamp, Last.fm (built-in app key), and MusicBrainz search. Drill
  into artists, albums, and playlists.
- **Custom providers** — Add your own Subsonic-compatible server (Navidrome, Airsonic, Gonic) or Jellyfin server in Settings, test the connection, and use it for search, playback, downloads, radio, browsing, and artist pages.
- **Streaming & caching** — Stream audio via yt-dlp (YouTube/SoundCloud) or direct HTTP (Bandcamp), with fully native decoding, cached to disk for instant replay.
- **Downloads** — Download tracks to MP3 via yt-dlp, with a Downloads view and on-row indicators.
- **Local music & playlists** — Add local files (MP3, FLAC, WAV, OGG, M4A, AAC, OPUS, WMA) and create, rename, delete, and organize playlists.
- **Library** — Save albums, artists, and playlists; browse them from the sidebar.
- **Artist pages** — Header with stats plus Most popular, Albums, Playlists, and Fans-also-like sections, each with its own provider picker.
- **Radio** — Song radio and artist radio from search results.
- **Queue** — Queue panel with Up Next and Recently Played tabs.
- **Trashbin** — Trash tracks from the context menu; trashed tracks are skipped at track-change time, marked with an icon, and listed in the sidebar Trashbin view.
- **Split panes** — Split the main view.
- **Drag & drop** — Drag tracks between views, into the queue, onto playlists (reorder or turn a card into a local playlist), and into the
  Library.
- **Lyrics** — Free, no-key LRCLib lyrics with synced lines that seek on click, plus Genius with per-line annotations, the song blurb, and a language picker row for its translations (loaded on demand); cached per track. Add your own named custom lyrics per track (plain or `[mm:ss.xx]`-synced) — they appear as tabs next to the providers, editable and deletable from the lyrics view. Attach multi-line notes to any custom line with `#` comment lines; the active line's note shows in an editable block below the lyrics. **Translate with AI** — translate the active lyrics (with learner notes) via any OpenAI-compatible server or local Ollama; the API key, server URL, and model live in Settings, and the result opens in the lyrics editor for review.
- **Media controls** — OS media keys / MPRIS (Linux) / SMTC (Windows) / Now Playing (macOS) via souvlaki.
- **More** — Search history, volume normalization, navigation history, session restore, right-click context menus (tracks, and artist/album/playlist cards for open / open in browser / save as playlist), dark theme.
- **Localization** — 13 languages: English, Polski, Español, Português (Brasil), 简体中文, العربية, Беларуская, Français, Deutsch, 日本語, Русский, हिन्दी, Українська.

## Installation

### Quick install (Linux / macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/awfghHJVBKLDHIOWAEFGUH/music_plr/release-prep/tiny-skia-v1.5.1/scripts/install.sh | bash
```

Detects your platform, downloads the latest release, and installs everything. On Linux it also sets up the desktop entry and icon.

### Windows

Download the [latest release](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/releases/latest/download/goosemusic-x86_64-pc-windows-msvc.zip) and extract to a folder. Run `goosemusic.exe`.

### Manual install

Download from [GitHub releases](https://github.com/awfghHJVBKLDHIOWAEFGUH/music_plr/releases) or compile from source:

```sh
cargo install goosemusic
```

#### Linux desktop integration

The release tarball includes a `.desktop` file and icon. After extracting:

```sh
cp goosemusic ~/.local/bin/
cp goosemusic.desktop ~/.local/share/applications/
cp icons/logo.svg ~/.local/share/icons/goosemusic.svg
```

#### macOS

Open the architecture-matched `.dmg` and drag `Goosemusic.app` to Applications, or extract the portable `.app.tar.gz` archive. On first open, right-click → Open if macOS blocks it (unsigned app).

## Supported languages

English (`en`), Polski (`pl`), Español (`es`), Português (Brasil) (`pt_br`), 简体中文 (`zh_cn`), العربية (`ar`), Беларуская (`be`), Français (`fr`), Deutsch (`de`), 日本語 (`ja`), Русский (`ru`), हिन्दी (`hi`), Українська (`uk`).

Pick a language from the in-app **Settings** view. To add one, copy `src/i18n/en.rs` to a new module, translate the strings, and append one entry to the `languages!` macro in `src/i18n/mod.rs` — the `Language` enum and picker are generated from that list.

## Install & run

**Prerequisites**

- **Rust** (stable, edition 2021)
- **yt-dlp** — YouTube audio streaming and downloads
- **Python 3** + `ytmusicapi` — YouTube Music search (optional; falls back to yt-dlp)
- **D-Bus** session bus (Linux) — for MPRIS
- Network access — lyrics fetch live from [LRCLib](https://lrclib.net) and [Genius](https://genius.com) (no keys)

```sh
cargo build
cargo run
```

## Keyboard shortcuts

| Key                                                                                                      | Action                                                                                         |
| -------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| <kbd>Space</kbd>                                                                                         | Toggle play/pause                                                                              |
| <kbd>Esc</kbd>                                                                                           | Close in-list search → close search history → clear selection → return to Search               |
| <kbd>Delete</kbd>                                                                                        | Delete selected tracks (playlist/Downloads/Trashbin views)                                     |
| <kbd>←</kbd>/<kbd>→</kbd> or <kbd>h</kbd>/<kbd>l</kbd>                                                   | Move focus between queue panel and track list                                                  |
| <kbd>↑</kbd>/<kbd>↓</kbd> or <kbd>k</kbd>/<kbd>j</kbd>                                                   | Move through the focused list (auto-scrolls, wraps)                                            |
| <kbd>gg</kbd> / <kbd>G</kbd>, <kbd>Home</kbd>/<kbd>End</kbd>                                             | First / last row                                                                               |
| <kbd>PgUp</kbd>/<kbd>PgDn</kbd>, <kbd>Ctrl</kbd>+<kbd>U</kbd>/<kbd>D</kbd>                               | Page / half-page                                                                               |
| <kbd>Shift</kbd>+<kbd>↑</kbd>/<kbd>↓</kbd> (<kbd>J</kbd>/<kbd>K</kbd>), <kbd>Ctrl</kbd>+<kbd>Space</kbd> | Extend selection / toggle selection on focused row                                             |
| <kbd>Ctrl</kbd>+<kbd>F</kbd>                                                                             | In-list fuzzy search over the hovered track list (<kbd>n</kbd>/<kbd>p</kbd> step matches)      |
| <kbd>Enter</kbd>                                                                                         | Play the focused (or hovered) track                                                            |
| <kbd>Ctrl</kbd>+<kbd>C</kbd> / <kbd>Ctrl</kbd>+<kbd>V</kbd>                                              | Copy / paste selected tracks                                                                   |
| <kbd>Ctrl</kbd>+<kbd>A</kbd>                                                                             | Select all tracks in the focused list                                                          |
| <kbd>Alt</kbd>+<kbd>P</kbd> / <kbd>Alt</kbd>+<kbd>1..5</kbd>, <kbd>Alt</kbd>+<kbd>S</kbd>                | Stage search provider / scope (pending until <kbd>Enter</kbd>)                                 |
| <kbd>Alt</kbd>+<kbd>Enter</kbd>                                                                          | Run the pending search from anywhere                                                           |
| <kbd>Ctrl</kbd>+<kbd>K</kbd>                                                                             | Playlist picker: fuzzy search over playlists (<kbd>Shift</kbd>+<kbd>Enter</kbd> plays)         |
| <kbd>Alt</kbd>+<kbd>↑</kbd>/<kbd>↓</kbd>                                                                 | Previous / next playlist                                                                       |
| <kbd>Alt</kbd>+<kbd>←</kbd>/<kbd>→</kbd>                                                                 | Back / forward in the focused pane                                                             |
| <kbd>N</kbd> / <kbd>P</kbd>                                                                              | Next / previous track                                                                          |
| <kbd>Q</kbd> / <kbd>R</kbd> / <kbd>Shift</kbd>+<kbd>L</kbd>                                              | Toggle queue / repeat / lyrics                                                                 |
| <kbd>T</kbd>                                                                                             | Switch queue panel tab: Queue ↔ Recently Played (opens panel if hidden)                        |
| <kbd>M</kbd>, <kbd>-</kbd>/<kbd>=</kbd>, <kbd>,</kbd>/<kbd>.</kbd>                                       | Mute, volume, seek ∓5s (<kbd>Shift</kbd> ∓10s)                                                 |
| <kbd>\</kbd> / <kbd>Shift</kbd>+<kbd>\</kbd>                                                             | Split focused pane side by side / stacked                                                      |
| <kbd>Ctrl</kbd>+<kbd>W</kbd>                                                                             | Close the focused pane                                                                         |
| <kbd>Ctrl</kbd>+<kbd>←</kbd>/<kbd>→</kbd>/<kbd>↑</kbd>/<kbd>↓</kbd>                                      | Move focus to the adjacent pane                                                                |
| <kbd>Ctrl</kbd>+<kbd>Tab</kbd>, <kbd>Ctrl</kbd>+<kbd>1..4</kbd>                                          | Cycle / jump between panes                                                                     |
| <kbd>?</kbd> / <kbd>F1</kbd> / <kbd>Ctrl</kbd>+<kbd>P</kbd>                                              | Command palette: commands, views, library, playlists (<kbd>Shift</kbd>+<kbd>Enter</kbd> plays) |

Every runnable action above is also listed by name in the command palette (`?`, `F1` or `Ctrl+P`), with its key chord; `Ctrl+K` opens the playlist-only picker.

## Configuration

Config lives at `~/.config/goosemusic/config.json` and is also editable live from the in-app **Settings** view.

| Field                        | Description                                               | Default              |
| ---------------------------- | --------------------------------------------------------- | -------------------- |
| `download_dir`               | Download directory                                        | `~/Music/goosemusic` |
| `cache_max_size_mb`          | Max stream cache size (MB)                                | `1024`               |
| `max_search_history_stored`  | Search history entries kept on disk                       | `100`                |
| `max_search_history_visible` | Entries shown in the dropdown                             | `10`                 |
| `max_recently_played`        | Tracks kept in Recently Played                            | `50`                 |
| `volume_normalization`       | Consistent loudness across tracks                         | `false`              |
| `cookie_browser`             | Browser yt-dlp reads cookies from (age-restricted videos) | `none`               |

Persistent data goes to `~/.local/share/goosemusic` (playlists, library, downloads, trashbin, search history); regenerable caches (session, streamed audio,
thumbnails, lyrics) go to `~/.cache/goosemusic`.

## Technical notes

**State** — All state lives in one `MusicPlayer`; `view()` is a pure function of `&MusicPlayer`. Background work (search, download, thumbnails)
returns via an mpsc channel drained by a 1Hz tick (none when idle). Stores implement a `JsonStore` trait and degrade to defaults on failure.

**Audio** — A dedicated output thread runs the rodio sink. yt-dlp streams AAC-in-M4A into a still-growing cache file; a custom
`SymphoniaStreamingSource` over a non-seekable `GrowingMediaSource` decodes sequentially so playback starts within a few KB.
Cached/downloaded/local files use the same source in seekable mode. Per-track normalization gain is computed once (symphonia) and applied on
replay.

## License

MIT
