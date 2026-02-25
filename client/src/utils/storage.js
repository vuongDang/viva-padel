import * as SecureStore from 'expo-secure-store';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { Platform } from 'react-native';

const isWeb = Platform.OS === 'web';

/**
 * A cross-platform storage utility that uses SecureStore on native 
 * and AsyncStorage on web.
 */
export const storage = {
    /**
     * Store a value.
     */
    setItem: async (key, value) => {
        try {
            if (isWeb) {
                await AsyncStorage.setItem(key, value);
            } else {
                await SecureStore.setItemAsync(key, value);
            }
        } catch (error) {
            console.error(`[Storage] Error setting item ${key}:`, error);
        }
    },

    /**
     * Get a value.
     */
    getItem: async (key) => {
        try {
            if (isWeb) {
                return await AsyncStorage.getItem(key);
            } else {
                return await SecureStore.getItemAsync(key);
            }
        } catch (error) {
            console.error(`[Storage] Error getting item ${key}:`, error);
            return null;
        }
    },

    /**
     * Delete a value.
     */
    deleteItem: async (key) => {
        try {
            if (isWeb) {
                await AsyncStorage.removeItem(key);
            } else {
                await SecureStore.deleteItemAsync(key);
            }
        } catch (error) {
            console.error(`[Storage] Error deleting item ${key}:`, error);
        }
    }
};
