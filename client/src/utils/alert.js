import { Alert, Platform } from 'react-native';

export const showAlert = (title, message, buttons) => {
    if (Platform.OS === 'web') {
        const alertButtons = buttons && buttons.length > 0 ? buttons : [{ text: 'OK' }];

        // If there is only one button, use window.alert
        if (alertButtons.length === 1) {
            window.alert([title, message].filter(Boolean).join('\n'));
            if (alertButtons[0].onPress) {
                alertButtons[0].onPress();
            }
            return;
        }

        // If there are multiple buttons (e.g., Cancel & OK), use window.confirm
        const result = window.confirm([title, message].filter(Boolean).join('\n'));

        if (result) {
            // Find the confirm button (usually not 'cancel' style or the last button)
            const confirmButton = alertButtons.find(({ style }) => style !== 'cancel') || alertButtons[alertButtons.length - 1];
            if (confirmButton && confirmButton.onPress) {
                confirmButton.onPress();
            }
        } else {
            // Find the cancel button
            const cancelButton = alertButtons.find(({ style }) => style === 'cancel') || alertButtons[0];
            if (cancelButton && cancelButton.onPress) {
                cancelButton.onPress();
            }
        }
    } else {
        Alert.alert(title, message, buttons);
    }
};
