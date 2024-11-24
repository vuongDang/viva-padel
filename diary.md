# Diary

## Build

### fatal error C1056: cannot update the time date stamp field

- Only in Windows and due to Leptos
- This is due to a dependency that can not compile to `wasm32-unknown-unknown`
- Suspicions is that leptos build the frontend to wasm32 but also the backend dependencies
  - the backend dependencies may not be compatible to wasm32 explaining where the error comes from
- A fix is to build the app backend separately and removing `trunk` influence
  - in `tauri.conf.json` remove the lines
  ```
    "beforeDevCommand": "trunk serve",
    "beforeBuildCommand": "trunk build",
  ```
  - build backend then you can relaunch
- https://github.com/tauri-apps/tauri/issues/10926

### Windows only: system error 0x5

- Windows defender will prevent `cargo` from writing to the filesystem
- add directory to the exclusions
- be careful when you remove a directory the exclusion is not working anymore even if you recreate the same directory
  - for example for `target/debug` with `cargo clean`

## Server responses

- Sometimes the server put some courts as `bookable: false`
  - not sure yet why we should check with official app

## Rust

### Tracing

- the default with default `FormatSubscriber` for `#[instrument]` is not to log any span but only event
  - can be solved with the `with_span_event` in the builder
- spans lifecycle: new, (enter, exit)\*, close
  - a span can be entered and exited multiple times when the execution thread change context (specific to async)
