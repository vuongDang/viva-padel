import * as Notifications from 'expo-notifications';
import * as Device from 'expo-device';
import Constants from 'expo-constants';
import { Platform } from 'react-native';
import * as SecureStore from 'expo-secure-store';
import { fetchWithTimeout } from '../utils/apiUtils';

const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

const LAST_TOKEN_KEY = 'viva_padel_last_notif_token';
const LAST_USER_KEY = 'viva_padel_last_notif_user';
const INSTALLATION_ID_KEY = 'viva_padel_installation_id';

/**
 * Generates a pseudo-random UUID v4.
 */
function generateUUID() {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
        const r = Math.random() * 16 | 0;
        const v = c === 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}

// This handler determines how to show a notification when the app is in the FOREGROUND.
// When the app is in the BACKGROUND or CLOSED, the OS handles the display automatically.
Notifications.setNotificationHandler({
    handleNotification: async () => ({
        shouldShowAlert: true,
        shouldPlaySound: true,
        shouldSetBadge: true,
    }),
});

const API_URL = process.env.EXPO_PUBLIC_API_URL;

export const NotificationService = {
    /**
     * Gets or creates a persistent unique installation ID for this device.
     */
    getInstallationId: async () => {
        try {
            let id = await SecureStore.getItemAsync(INSTALLATION_ID_KEY);
            if (!id) {
                id = generateUUID();
                await SecureStore.setItemAsync(INSTALLATION_ID_KEY, id);
                console.log('[NotificationService] Generated new installation ID:', id);
            }
            return id;
        } catch (e) {
            console.error('[NotificationService] Failed to get/set installation ID:', e);
            return Device.osBuildId || 'unknown_device';
        }
    },

    /**
     * Registers the device for push notifications and returns the token.
     * This is required for your backend to target this specific device.
     */
    registerForPushNotificationsAsync: async () => {
        let token;

        // Detect if we are running in Expo Go
        const isExpoGo = Constants.appOwnership === 'expo';
        if (isExpoGo) {
            console.log('[NotificationService] Running in Expo Go. Will attempt to get Expo token for testing.');
            // We continue instead of returning null to allow testing the registration flow
        }

        if (!Device.isDevice) {
            console.warn('[NotificationService] Not a physical device. Registration might fail or return a simulation token.');
            // For testing, we might still want to proceed if we can get a token
        }

        const { status: existingStatus } = await Notifications.getPermissionsAsync();
        let finalStatus = existingStatus;

        if (existingStatus !== 'granted') {
            const { status } = await Notifications.requestPermissionsAsync();
            finalStatus = status;
        }

        if (finalStatus !== 'granted') {
            console.error('[NotificationService] Permission for push notifications not granted. Status:', finalStatus);
            return null;
        }

        const maxAttempts = 3;
        for (let attempt = 1; attempt <= maxAttempts; attempt++) {
            try {
                // In Expo SDK 50+, projectId is required. We pull it from expoConfig.
                const projectId = Constants?.expoConfig?.extra?.eas?.projectId ?? Constants?.easConfig?.projectId;

                if (!projectId) {
                    console.error('[NotificationService] Project ID not found in app config. Ensure EAS is configured.');
                    return null;
                }

                console.log(`[NotificationService] Attempting to fetch push token (Attempt ${attempt}/${maxAttempts})...`);

                token = (await Notifications.getExpoPushTokenAsync({
                    projectId: projectId,
                })).data;

                if (token) {
                    console.log('--- DEVICE PUSH TOKEN ---');
                    console.log(token);
                    console.log('-------------------------');
                    break; // Success!
                }
            } catch (e) {
                console.error(`[NotificationService] Error fetching push token (Attempt ${attempt}):`, e);

                // If it's the last attempt, log the full error object for debugging
                if (attempt === maxAttempts) {
                    try {
                        console.error('[NotificationService] Final registration error details:', JSON.stringify(e, null, 2));
                    } catch (err) {
                        console.error('[NotificationService] Could not stringify final error object.');
                    }
                } else {
                    // Wait before retrying (exponential backoff: 1s, 2s)
                    const waitTime = attempt * 1000;
                    console.log(`[NotificationService] Retrying in ${waitTime}ms...`);
                    await delay(waitTime);
                }
            }
        }

        // Android specific configuration for priority and styling
        if (Platform.OS === 'android') {
            Notifications.setNotificationChannelAsync('default', {
                name: 'default',
                importance: Notifications.AndroidImportance.MAX,
                vibrationPattern: [0, 250, 250, 250],
                lightColor: '#FF231F7C',
            });
        }

        return token;
    },

    /**
     * Registers the device with the backend server.
     * @param {string} pushToken The push token (Expo or native) 
     * @param {string} userToken The authentication token for the current user
     * @param {boolean} force If true, bypass local cache and register even if it seems already done
     */
    registerDeviceWithServer: async (pushToken, userToken, userEmail, force = false) => {
        if (!pushToken || !userToken || !userEmail) {
            console.warn('[NotificationService] Missing pushToken, userToken or userEmail for registration');
            return;
        }

        // Optimization: check if we already registered this token for this user
        if (!force) {
            try {
                const lastToken = await SecureStore.getItemAsync(LAST_TOKEN_KEY);
                const lastUser = await SecureStore.getItemAsync(LAST_USER_KEY);

                if (lastToken === pushToken && lastUser === userEmail) {
                    // Log only in dev or via hidden console to avoid cluttering normal startup
                    if (__DEV__) console.log('[NotificationService] Device registration matches cache. Skipping server call.');
                    return;
                }
            } catch (e) {
                console.warn('[NotificationService] Failed to read registration cache:', e);
            }
        } else {
            console.log('[NotificationService] Requesting registration sync (bypass cache).');
        }

        const deviceId = await NotificationService.getInstallationId();

        try {
            console.log('[NotificationService] Registering device with server...', { deviceId, pushToken, userEmail });
            const response = await fetchWithTimeout(`${API_URL}/register-device`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${userToken}`,
                    'CF-Access-Client-Id': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID,
                    'CF-Access-Client-Secret': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET,
                },
                body: JSON.stringify({
                    notif_token: pushToken,
                    device_id: deviceId,
                }),
            });

            if (!response.ok) {
                const errorText = await response.text();
                console.error('[NotificationService] Registration failed:', errorText);
            } else {
                console.log('[NotificationService] Device registered successfully');
                // Cache the registration
                try {
                    await SecureStore.setItemAsync(LAST_TOKEN_KEY, pushToken);
                    await SecureStore.setItemAsync(LAST_USER_KEY, userEmail);
                } catch (e) {
                    console.warn('[NotificationService] Failed to write registration cache:', e);
                }
            }
        } catch (error) {
            console.error('[NotificationService] Network error during registration:', error);
        }
    },

    /**
     * Sets up listeners for notifications.
     * - onReceived: Triggered when a notification arrives while app is open.
     * - onResponse: Triggered when a user taps a notification (even if app was closed).
     */
    initListeners: (onReceived, onResponse) => {
        const notificationListener = Notifications.addNotificationReceivedListener(notification => {
            if (onReceived) onReceived(notification);
        });

        const responseListener = Notifications.addNotificationResponseReceivedListener(response => {
            if (onResponse) onResponse(response);
        });

        // In modern Expo, notification responses that open the app are handled
        // via the responseListener or the useLastNotificationResponse hook in App.js.

        return () => {
            notificationListener.remove();
            responseListener.remove();
        };
    }
};
