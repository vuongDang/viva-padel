import { CONFIG } from '../config';

const unauthorizedListeners = new Set();
export const onUnauthorized = (callback) => {
    unauthorizedListeners.add(callback);
    return () => unauthorizedListeners.delete(callback);
};

const notifyUnauthorized = () => {
    unauthorizedListeners.forEach(cb => cb());
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
