# Skippy Adaptive Comms Demo

Plain-text trace-id demo showing how adaptive banter selection can be driven by
tracked protocol files instead of purely ad-hoc prompting.

## What It Shows

- relationship phase as a traceable identifier
- cue-specific case files for different Joe situations
- jab family, boast, and lament choices emitted as `publish` lines
- downstream selections represented as `subscribe` lines
- tone activation represented as `trigger` lines

The demo uses the real `recur trace-id` command over `.txt` files.

## Local Config

Repo `.recur/` paths are ignored, so this demo ships a tracked
`trace-id.config.example.toml` instead of a committed `.recur/config.toml`.

The demo script copies it into a local ignored `.recur/config.toml` if needed.

## Quick Start

```powershell
powershell -ExecutionPolicy Bypass -File demos/skippy-adaptive-comms/demo.ps1
```
