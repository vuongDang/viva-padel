import * as Notifications from 'expo-notifications';
import * as Device from 'expo-device';
import Constants from 'expo-constants';
import { Platform } from 'react-native';

// This handler determines how to show a notification when the app is in the FOREGROUND.
// When the app is in the BACKGROUND or CLOSED, the OS handles the display automatically.
Notifications.setNotificationHandler({
    handleNotification: async () => ({
        shouldShowAlert: true,
        shouldPlaySound: true,
        shouldSetBadge: true,
    }),
});

export const NotificationService = {
    /**
     * Registers the device for push notifications and returns the token.
     * This is required for your backend to target this specific device.
     */
    registerForPushNotificationsAsync: async () => {
        let token;

        // Detect if we are running in Expo Go
        const isExpoGo = Constants.appOwnership === 'expo';
        if (isExpoGo) {
            console.warn('Push Notifications (FCM) are not supported in Expo Go. Use a Development Build to test this feature.');
            return null;
        }

        if (!Device.isDevice) {
            console.warn('Push Notifications require a physical device');
            return null;
        }

        const { status: existingStatus } = await Notifications.getPermissionsAsync();
        let finalStatus = existingStatus;

        if (existingStatus !== 'granted') {
            const { status } = await Notifications.requestPermissionsAsync();
            finalStatus = status;
        }

        if (finalStatus !== 'granted') {
            console.error('Failed to get push token for push notification!');
            return null;
        }

        try {
            // In Expo SDK 50+, projectId is required. We pull it from expoConfig.
            const projectId = Constants?.expoConfig?.extra?.eas?.projectId ?? Constants?.easConfig?.projectId;

            if (!projectId) {
                throw new Error('Project ID not found in app config. Ensure EAS is configured.');
            }

            token = (await Notifications.getExpoPushTokenAsync({
                projectId: projectId,
            })).data;

            console.log('--- DEVICE PUSH TOKEN ---');
            console.log(token);
            console.log('-------------------------');
        } catch (e) {
            console.error('Error fetching push token:', e);
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
