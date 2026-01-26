# Viva Padel

A personal application used to book padel courts at LeGarden, Rennes. It includes a Rust backend for scanning availabilities and a React Native (Expo) mobile frontend.

## 🚀 Backend Server

The server is built with Rust and handles the logic for scanning cancelled slots and providing availability data.

### Running with Cargo
From the root directory:
```bash
cargo run --bin viva-padel-server
```
Or from the `server` directory:
```bash
cargo run
```

### Running with Docker
Build and run using Docker Compose:
```bash
docker-compose up --build
```
Or manually with Docker:
```bash
docker build -t viva-padel-server .
docker run -p 3000:3000 viva-padel-server
```

---

## 📱 Mobile Application

The mobile app is built with React Native and Expo.

### Development Mode
1. Navigate to the mobile directory: `cd mobile`
2. Start the development server:
   ```bash
   npx expo start
   ```
3. **Choose your device:**
   - **USB**: Plug in your Android/iOS device and press `a` (Android) or `i` (iOS) in the terminal.
   - **Expo Go**: Scan the QR code with your phone (Android) or Camera app (iOS).
   - **Tunnel**: If you are not on the same Wi-Fi, use:
     ```bash
     npx expo start --tunnel
     ```

### Environment Variables
Ensure you have a `.env` file in the `mobile` directory (see `.env.example`).
Variables must be prefixed with `EXPO_PUBLIC_` to be loaded by Expo.

---

## 📦 Distribution (EAS Build & Updates)

To build a standalone version of the app and push live updates without re-installing the APK.

### 1. Prerequisites
- Install EAS CLI: `npm install -g eas-cli`
- Login: `eas login`
- Init project (first time): `eas init`

### 2. Configure Environment Variables
EAS needs your Cloudflare secrets for the build. Use the following commands (standard `eas secret` or the newer `eas env`):
```bash
eas env:create preview --name EXPO_PUBLIC_CF_ACCESS_CLIENT_ID --value "your-id" --type string --visibility secret
eas env:create preview --name EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET --value "your-secret" --type string --visibility secret
eas env:create preview --name EXPO_PUBLIC_API_URL --value "your-server-url" --type string --visibility sensitive
```

### 3. Build & Update Workflow

#### A. Generate a new APK (Major changes)
Use this when you change the app icon, splash screen, or native modules:
```bash
eas build --platform android --profile preview
```

#### B. Push Live Updates (Quick fixes)
Use this for UI changes or logic fixes. It updates the app instantly for anyone who already has the APK installed:
```bash
eas update --branch preview --message "Your description"
```


---

## ⚠️ Deprecated

### `tauri_app`
The `tauri_app` folder contains a legacy desktop version of this project. It is currently **deprecated** and no longer maintained. Use the `mobile` app for the best experience.
