# Small projects (elegant code)

Each subfolder can be its own Cargo crate when you outgrow single-file demos.

## Ideas (pick one at a time)

1. **CLI file stats** — walk a directory, print sizes, use `Result` and `Path`.
2. **INI / JSON mini parser** — enums + pattern matching, no full serde requirement at first.
3. **In-memory key-value store** — `HashMap`, tests, error types.
4. **Thread pool worker** — bounded channel, graceful shutdown (later syllabus).
5. **Tiny HTTP or TCP echo** — async crate when you reach topic 29.

## Project template

When you start a project:

```text
projects/
  p01-<name>/
    Cargo.toml
    src/
      main.rs
```

Keep the README here updated with a one-line description per project and what you learned.
