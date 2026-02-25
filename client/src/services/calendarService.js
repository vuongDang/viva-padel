import { fetchWithTimeout } from '../utils/apiUtils';
import { Platform } from 'react-native';

const API_URL = process.env.EXPO_PUBLIC_API_URL;

export const CalendarService = {
    /**
     * Fetch calendar availabilities.
     */
    fetchReservations: async () => {
        try {
            const headers = {
                "Content-Type": "application/json",
            };

            // Only append Cloudflare Access headers if not on web
            if (Platform.OS !== 'web') {
                if (process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID) {
                    headers['CF-Access-Client-Id'] = process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID;
                }
                if (process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET) {
                    headers['CF-Access-Client-Secret'] = process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET;
                }
            }

            const response = await fetchWithTimeout(`${API_URL}/calendar`, {
                method: "GET",
                headers,
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(errorText || 'Failed to fetch availabilities');
            }

            return await response.json();
        } catch (error) {
            console.error('[CalendarService] Fetch Reservations Error:', error);
            throw error;
        }
    }
};
