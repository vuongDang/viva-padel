import { CONFIG } from '../config';

/**
 * Fetch with a default timeout.
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
        return response;
    } catch (error) {
        clearTimeout(id);
        throw error;
    }
}
