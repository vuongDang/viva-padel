import * as SecureStore from 'expo-secure-store';
import { fetchWithTimeout } from '../utils/apiUtils';

const API_URL = process.env.EXPO_PUBLIC_API_URL;
const TOKEN_KEY = 'viva_padel_auth_token';
const EMAIL_KEY = 'viva_padel_user_email';

export const AuthService = {
    /**
     * Signup a new user.
     * @param {string} email 
     */
    signup: async (email) => {
        try {
            const response = await fetchWithTimeout(`${API_URL}/signup`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'CF-Access-Client-Id': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID,
                    'CF-Access-Client-Secret': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET,
                },
                body: JSON.stringify({ email }),
            });

            if (!response.ok) {
                const errorText = await response.text();
                let errorMessage = 'Failed to signup';
                try {
                    const errorJson = JSON.parse(errorText);
                    errorMessage = errorJson.error || errorMessage;
                } catch (e) {
                    errorMessage = errorText || errorMessage;
                }
                throw new Error(errorMessage);
            }

            return await response.json();
        } catch (error) {
            console.error('[AuthService] Signup Error:', error);
            throw error;
        }
    },

    /**
     * Login an existing user and store the token.
     * @param {string} email 
     */
    login: async (email) => {
        try {
            const response = await fetchWithTimeout(`${API_URL}/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'CF-Access-Client-Id': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID,
                    'CF-Access-Client-Secret': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET,
                },
                body: JSON.stringify({ email }),
            });

            if (!response.ok) {
                const errorText = await response.text();
                let errorMessage = 'Failed to login';
                try {
                    const errorJson = JSON.parse(errorText);
                    errorMessage = errorJson.error || errorMessage;
                } catch (e) {
                    errorMessage = errorText || errorMessage;
                }
                throw new Error(errorMessage);
            }

            const data = await response.json();
            if (data.token) {
                await SecureStore.setItemAsync(TOKEN_KEY, data.token);
                await SecureStore.setItemAsync(EMAIL_KEY, email);
            }
            return data;
        } catch (error) {
            console.error('[AuthService] Login Error:', error);
            throw error;
        }
    },

    /**
     * Get the stored token.
     */
    getToken: async () => {
        try {
            return await SecureStore.getItemAsync(TOKEN_KEY);
        } catch (error) {
            console.error('[AuthService] Get Token Error:', error);
            return null;
        }
    },

    /**
     * Get the stored user email.
     */
    getEmail: async () => {
        try {
            return await SecureStore.getItemAsync(EMAIL_KEY);
        } catch (error) {
            console.error('[AuthService] Get Email Error:', error);
            return null;
        }
    },

    /**
     * Get user profile and alarms.
     * @param {string} token 
     */
    getUserInfo: async (token) => {
        try {
            const response = await fetchWithTimeout(`${API_URL}/user`, {
                method: 'GET',
                headers: {
                    'Authorization': `Bearer ${token}`,
                    'CF-Access-Client-Id': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID,
                    'CF-Access-Client-Secret': process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET,
                },
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(errorText || 'Failed to fetch user info');
            }

            return await response.json();
        } catch (error) {
            console.error('[AuthService] Get User Info Error:', error);
            throw error;
        }
    },

    /**
     * Logout and clear storage.
     */
    logout: async () => {
        try {
            await SecureStore.deleteItemAsync(TOKEN_KEY);
            await SecureStore.deleteItemAsync(EMAIL_KEY);
        } catch (error) {
            console.error('[AuthService] Logout Error:', error);
        }
    },
};
