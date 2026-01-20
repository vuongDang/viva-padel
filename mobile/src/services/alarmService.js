import { AuthService } from "./authService";

const API_URL = process.env.EXPO_PUBLIC_API_URL;

const WEEKDAY_MAP = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

export const AlarmService = {
    async syncAlarms(alarms) {
        const token = await AuthService.getToken();
        if (!token) throw new Error("Veuillez vous connecter pour activer les notifications.");

        const mappedAlarms = alarms.map(alarm => {
            // Mapping CourtType
            let court_type = "Both";
            if (alarm.types.indoor && !alarm.types.outdoor) court_type = "Indoor";
            else if (!alarm.types.indoor && alarm.types.outdoor) court_type = "Outdoor";

            return {
                name: alarm.name,
                days_of_the_week: (alarm.weekdays || []).map(d => WEEKDAY_MAP[d]),
                time_range: [
                    `${alarm.startTime || "00:00"}:00`,
                    `${alarm.endTime || "23:59"}:00`
                ],
                court_type: court_type,
                weeks_ahead: alarm.period || 1,
                is_active: alarm.activated ?? true
            };
        });

        try {
            const response = await fetch(`${API_URL}/alarms`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`,
                    'CF-Access-Client-Id': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID,
                    'CF-Access-Client-Secret': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET,
                },
                body: JSON.stringify({ alarms: mappedAlarms }),
            });

            if (!response.ok) {
                const errorText = await response.text();
                let errorMessage = "Erreur lors de la synchronisation";
                try {
                    const errorJson = JSON.parse(errorText);
                    errorMessage = errorJson.error || errorMessage;
                } catch (e) { }
                throw new Error(errorMessage);
            }

            return true;
        } catch (error) {
            console.error('[AlarmService] Sync Error:', error);
            throw error;
        }
    },

    mapServerAlarmsToMobile(serverAlarms) {
        if (!serverAlarms) return [];

        const REVERSE_WEEKDAY_MAP = {
            "Mon": 0, "Tue": 1, "Wed": 2, "Thu": 3, "Fri": 4, "Sat": 5, "Sun": 6
        };

        return serverAlarms.map((sa, index) => {
            // Map CourtType back
            const types = {
                indoor: sa.court_type === "Indoor" || sa.court_type === "Both",
                outdoor: sa.court_type === "Outdoor" || sa.court_type === "Both"
            };

            // Extract times (strip seconds)
            const startTime = (sa.time_range[0] || "00:00").substring(0, 5);
            const endTime = (sa.time_range[1] || "23:59").substring(0, 5);

            return {
                id: `server-${index}-${Date.now()}`,
                name: sa.name || `Alarme ${index + 1}`,
                weekdays: (sa.days_of_the_week || []).map(day => REVERSE_WEEKDAY_MAP[day]),
                startTime: startTime,
                endTime: endTime,
                types: types,
                period: sa.weeks_ahead || 1,
                activated: sa.is_active ?? true
            };
        });
    }
};
