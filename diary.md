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

## Server responses

- Sometimes the server put some courts as `bookable: false`
  - not sure yet why we should check with official app
