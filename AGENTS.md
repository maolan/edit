# Agent Instructions: `edit`

## Project Scope

`edit` is the Maolan audio editor. It is a standalone Rust crate that depends
on sibling Maolan crates by path:

- `../engine` for audio decode and OxideAV-backed audio encode helpers.
- `../widgets` for reusable iced widgets, including waveform display.

The editor is intentionally minimal for now: it opens audio files, displays the
waveform, and saves/exports the loaded audio. Do not add editing tools,
transport controls, effects, or timeline features unless explicitly requested.

## Build Profile

Always build, run, and test in debug mode. Do **not** pass `--release` to any
`cargo` command unless explicitly requested by the user.

## End-of-Change Routine

After every code change in this directory, run these commands in order:

```bash
cargo clippy --all-targets --fix --allow-dirty
cargo fmt
```

## Verification

For behavior changes, also run:

```bash
cargo check
cargo test --all-targets
```

## Clippy Warnings

If `cargo clippy --all-targets --fix --allow-dirty` does **not** automatically
fix all warnings, fix the remaining warnings manually.

- Do **not** use `#![allow(...)]` or `#[allow(...)]` directives to silence
  clippy warnings.
- Address the underlying issue reported by clippy.

Always ensure clippy and formatting complete successfully with no remaining
warnings before finishing.

## Audio I/O

Opening audio must use Maolan's shared decode path from
`maolan_engine::audio_codec`, so format support stays aligned with the DAW.

Saving audio must use Maolan's shared OxideAV-backed encode path:

- `AudioEncodeFormat::Wav` for WAV.
- `AudioEncodeFormat::Flac` for native FLAC.
- `AudioEncodeFormat::OggFlac` for Ogg FLAC.
- `AudioEncodeFormat::Mp3` for MP3.

Do not introduce a separate encoder stack in this crate without explicit
discussion.
