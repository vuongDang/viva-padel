import { apiFetch } from '../utils/apiUtils';

export const CalendarService = {
    /**
     * Fetch calendar availabilities.
     */
    fetchReservations: async () => {
        try {
            const response = await apiFetch('/calendar', { method: 'GET' });

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

