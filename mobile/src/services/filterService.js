import AsyncStorage from '@react-native-async-storage/async-storage';

const FILTERS_KEY = 'viva_padel_filters';
const ACTIVE_FILTER_KEY = 'viva_padel_active_filter';

export const FilterService = {
    async saveFilters(filters) {
        try {
            const jsonValue = JSON.stringify(filters);
            await AsyncStorage.setItem(FILTERS_KEY, jsonValue);
        } catch (e) {
            console.error('[FilterService] Failed to save filters:', e);
        }
    },

    async loadFilters() {
        try {
            const jsonValue = await AsyncStorage.getItem(FILTERS_KEY);
            return jsonValue != null ? JSON.parse(jsonValue) : null;
        } catch (e) {
            console.error('[FilterService] Failed to load filters:', e);
            return null;
        }
    },

    async saveActiveFilterId(id) {
        try {
            await AsyncStorage.setItem(ACTIVE_FILTER_KEY, id);
        } catch (e) {
            console.error('[FilterService] Failed to save active filter ID:', e);
        }
    },

    async loadActiveFilterId() {
        try {
            return await AsyncStorage.getItem(ACTIVE_FILTER_KEY);
        } catch (e) {
            console.error('[FilterService] Failed to load active filter ID:', e);
            return null;
        }
    },

    async clearFilters() {
        try {
            await AsyncStorage.removeItem(FILTERS_KEY);
            await AsyncStorage.removeItem(ACTIVE_FILTER_KEY);
        } catch (e) {
            console.error('[FilterService] Failed to clear filters:', e);
        }
    }
};
