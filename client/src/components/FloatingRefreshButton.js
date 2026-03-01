import React from 'react';
import { StyleSheet, TouchableOpacity, Text, ActivityIndicator, Platform } from 'react-native';

const FloatingRefreshButton = ({ onPress, loading, style }) => {
    return (
        <TouchableOpacity
            style={[styles.floatingRefreshButton, loading && styles.disabledButton, style]}
            onPress={onPress}
            disabled={loading}
        >

            {loading ? (
                <ActivityIndicator size="small" color="#FFF" />
            ) : (
                <Text style={styles.refreshBtnText}>Rafraîchir</Text>
            )}
        </TouchableOpacity>
    );
};

const styles = StyleSheet.create({
    floatingRefreshButton: {
        backgroundColor: '#1A1A1A',
        borderRadius: 25,
        paddingHorizontal: 16, // Unified padding
        height: 50,
        justifyContent: 'center',
        alignItems: 'center',
        ...Platform.select({
            web: {
                boxShadow: '0px 4px 4px rgba(0, 0, 0, 0.3)',
            },
            default: {
                elevation: 5,
                shadowColor: '#000',
                shadowOffset: { width: 0, height: 4 },
                shadowOpacity: 0.3,
                shadowRadius: 4,
            }
        }),
        zIndex: 10,
    },


    refreshBtnText: {
        fontSize: 15,
        fontWeight: '700',
        color: '#FFF',
        textAlign: 'center',
        includeFontPadding: false,
        textAlignVertical: 'center',
    },
    disabledButton: {
        opacity: 0.6,
    },
});

export default FloatingRefreshButton;
