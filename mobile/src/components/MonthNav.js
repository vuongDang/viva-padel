import React from 'react';
import { StyleSheet, View, Text, TouchableOpacity } from 'react-native';
import { theme } from '../styles/theme';

export default function MonthNav({ currentDate, onPrevMonth, onNextMonth }) {
    const monthLabel = currentDate.toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' });
    const capitalizedLabel = monthLabel.charAt(0).toUpperCase() + monthLabel.slice(1);

    return (
        <View style={styles.container}>
            <TouchableOpacity onPress={onPrevMonth} style={styles.navBtn}>
                <Text style={styles.navBtnText}>{'<'}</Text>
            </TouchableOpacity>

            <View style={styles.labelContainer}>
                <Text style={styles.labelText}>{capitalizedLabel}</Text>
            </View>

            <TouchableOpacity onPress={onNextMonth} style={styles.navBtn}>
                <Text style={styles.navBtnText}>{'>'}</Text>
            </TouchableOpacity>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'center',
        marginHorizontal: 16,
        marginBottom: 8,
        gap: 12,
    },

    navBtn: {
        width: 40,
        height: 40,
        borderRadius: 16,
        borderWidth: 1,
        borderColor: theme.colors.border,
        backgroundColor: theme.colors.background,
        alignItems: 'center',
        justifyContent: 'center',
    },
    navBtnText: {
        color: '#5f6368', // Hardcoded from css
        fontSize: 22,
        marginTop: -2, // Visual adjustment
    },
    labelContainer: {
        backgroundColor: theme.colors.background,
        paddingVertical: 8,
        paddingHorizontal: 16,
        borderRadius: 20,
        borderWidth: 1,
        borderColor: theme.colors.border,
        minWidth: 150,
        alignItems: 'center',
    },
    labelText: {
        fontSize: 18,
        fontWeight: '600',
        color: '#202124',
    }
});
