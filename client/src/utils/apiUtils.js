import { CONFIG } from "../config";
import { Platform } from "react-native";

const API_URL = process.env.EXPO_PUBLIC_API_URL;

const unauthorizedListeners = new Set();
export const onUnauthorized = (callback) => {
  unauthorizedListeners.add(callback);
  return () => unauthorizedListeners.delete(callback);
};

const notifyUnauthorized = () => {
  unauthorizedListeners.forEach((cb) => cb());
};

/**
 * Fetch with a default timeout and 401 detection.
 */
export async function fetchWithTimeout(url, options = {}) {
  const { timeout = CONFIG.API_TIMEOUT } = options;

  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, {
      ...options,
      signal: controller.signal,
    });
    clearTimeout(id);

    if (response.status === 401) {
      notifyUnauthorized();
    }

    return response;
  } catch (error) {
    clearTimeout(id);
    throw error;
  }
}

/**
 * Generic backend API caller.
 *
 * Builds the full URL as `${API_URL}${endpoint}` and automatically:
 *   - Sets Content-Type: application/json
 *   - Adds CF Access headers (Client-Id + Client-Secret) on non-web platforms
 *   - Merges any extra headers / options you pass in
 *
 * @param {string} endpoint  - Path relative to API_URL, e.g. "/calendar"
 * @param {RequestInit} options - fetch options (method, headers, body, …)
 * @returns {Promise<Response>}
 */
export async function apiFetch(endpoint, options = {}) {
  const { headers: extraHeaders = {}, ...restOptions } = options;

  const baseHeaders = {
    "Content-Type": "application/json",
    ...extraHeaders,
  };

  if (Platform.OS !== "web") {
    // For mobile we use authorization token
    if (process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID) {
      baseHeaders["CF-Access-Client-Id"] =
        process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID;
    }
    if (process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET) {
      baseHeaders["CF-Access-Client-Secret"] =
        process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET;
    }
  } else {
    // For web we reinject the token that was given when logged in
    const cf_token = getCloudflareToken();
    baseHeaders["Authorization"] = "Bearer ${cf_token}";
    // restOptions.credentials = "include";
  }

  return fetchWithTimeout(`${API_URL}${endpoint}`, {
    ...restOptions,
    headers: baseHeaders,
  });
}

// Get Cloudflare Token for web app
function getCloudflareToken() {
  if (Platform.OS == "web") {
    const name = "CF_Authorization=";
    const decodedCookie = decodeURIComponent(document.cookie);
    const ca = decodedCookie.split(";");
    for (let i = 0; i < ca.length; i++) {
      let c = ca[i].trim();
      if (c.indexOf(name) === 0) {
        return c.substring(name.length, c.length);
      }
    }
  }
  return "";
}
