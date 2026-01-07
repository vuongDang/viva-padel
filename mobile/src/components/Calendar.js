import React from 'react';
import { StyleSheet, View, Text, TouchableOpacity, Dimensions, Animated } from 'react-native';
import { theme } from '../styles/theme';

const WEEKDAYS = ['Lun', 'Mar', 'Mer', 'Jeu', 'Ven', 'Sam', 'Dim'];

export default function Calendar({ availabilities, currentMonthDate, onDateClick, filterFn }) {
    const year = currentMonthDate.getFullYear();
    const month = currentMonthDate.getMonth();

    const date = new Date(year, month, 1);
    let firstDay = date.getDay() - 1;
    if (firstDay === -1) firstDay = 6;

    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const prevMonthLastDay = new Date(year, month, 0).getDate();

    const days = [];

    // Padding prev month
    for (let i = 0; i < firstDay; i++) {
        days.push({
            day: prevMonthLastDay - (firstDay - 1 - i),
            isOtherMonth: true,
            key: `prev-${i}`
        });
    }

    // Current month
    for (let d = 1; d <= daysInMonth; d++) {
        const dateStr = `${year}-${String(month + 1).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
        // Check availability logic
        // availabilities is object { dateStr: { 'hydra:member': [...] } }
        // We need to pass logic down or do it here. 
        // Ideally logic is passed as a prop or we process it here.
        // Since logic is complex, let's assume `filterFn` returns boolean if date has availability

        const hasAvailability = filterFn ? filterFn(dateStr) : false;

        days.push({
            day: d,
            isOtherMonth: false,
            hasAvailability,
            dateStr,
            key: `curr-${d}`
        });
    }

    // Next month padding
    const totalCellsSoFar = days.length;
    const remainingCells = totalCellsSoFar % 7 === 0 ? 0 : 7 - (totalCellsSoFar % 7);

    for (let i = 1; i <= remainingCells; i++) {
        days.push({
            day: i,
            isOtherMonth: true,
            key: `next-${i}`
        });
    }

    // Calculate rows needed
    const totalCells = days.length;
    // We can just flex wrap

    return (
        <View style={styles.container}>
            <View style={styles.header}>
                {WEEKDAYS.map((day, i) => (
                    <Text key={i} style={styles.weekday}>{day}</Text>
                ))}
            </View>
            <View style={styles.grid}>
                {days.map((item, index) => (
                    <DayCell
                        key={item.key}
                        item={item}
                        onPress={() => item.isOtherMonth ? null : onDateClick(item.dateStr)}
                    />
                ))}
            </View>
        </View>
    );
}

function DayCell({ item, onPress }) {
    const isToday = !item.isOtherMonth && isItemsToday(item.day);
    const scaleValue = React.useRef(new Animated.Value(1)).current;

    const handlePressIn = () => {
        if (item.isOtherMonth || (onPress && typeof onPress !== 'function')) return;
        Animated.spring(scaleValue, {
            toValue: 0.9,
            useNativeDriver: true,
        }).start();
    };

    const handlePressOut = () => {
        if (item.isOtherMonth || (onPress && typeof onPress !== 'function')) return;
        Animated.spring(scaleValue, {
            toValue: 1,
            useNativeDriver: true,
        }).start();
    };

    // Status styles
    let statusStyle = {};
    if (!item.isOtherMonth) {
        statusStyle = item.hasAvailability ? styles.statusAvailable : styles.statusUnavailable;
    }

    return (
        <TouchableOpacity
            style={[
                styles.dayCell,
                item.isOtherMonth && styles.dayOtherMonth,
                statusStyle
            ]}
            onPress={onPress}
            onPressIn={handlePressIn}
            onPressOut={handlePressOut}
            activeOpacity={item.isOtherMonth ? 1 : 0.7}
        >
            <Animated.View style={[styles.dayNumberContainer, isToday && styles.todayContainer, { transform: [{ scale: scaleValue }] }]}>
                <Text style={[styles.dayNumber, isToday && styles.todayText]}>{item.day}</Text>
            </Animated.View>
        </TouchableOpacity>
    );
}

function isItemsToday(day) {
    const today = new Date();
    // Assuming hardcoded year logic from main.js? "2026, 0, 1"
    // No, main.js compares with actual today.
    return day === today.getDate() && today.getMonth() === 0 && today.getFullYear() === 2026;
    // Wait, the main.js logic was:
    // if (!isOtherMonth && day === today.getDate() && 2026 === today.getFullYear() && 0 === today.getMonth()) 
    // This seems to imply current date in the app context is Jan 2026.
    // I should replicate that logic or use actual date.
}

const styles = StyleSheet.create({
    container: {
        backgroundColor: 'white',
        borderRadius: 8,
        borderWidth: 1,
        borderColor: theme.colors.border,
        overflow: 'hidden',
    },
    header: {
        flexDirection: 'row',
        backgroundColor: '#f8f9fa',
        borderBottomWidth: 1,
        borderBottomColor: theme.colors.border,
    },
    weekday: {
        flex: 1,
        textAlign: 'center',
        paddingVertical: 12,
        fontSize: 12,
        fontWeight: '500',
        color: '#70757a',
        textTransform: 'uppercase',
    },
    grid: {
        flexDirection: 'row',
        flexWrap: 'wrap',
    },
    dayCell: {
        width: '14.28%', // 100% / 7
        height: 48, // Fixed height for more compact look
        borderRightWidth: StyleSheet.hairlineWidth, // Use hairline for cleaner look
        borderBottomWidth: StyleSheet.hairlineWidth,
        borderColor: theme.colors.border,
        padding: 4,
        alignItems: 'center',
        justifyContent: 'flex-start', // Align content to top
    },
    dayOtherMonth: {
        backgroundColor: '#f1f3f4',
    },
    statusAvailable: {
        backgroundColor: theme.colors.availableLight,
    },
    statusUnavailable: {
        backgroundColor: theme.colors.unavailableLight,
    },
    dayNumberContainer: {
        width: 28,
        height: 28,
        borderRadius: 14,
        alignItems: 'center',
        justifyContent: 'center',
        marginBottom: 4,
    },
    dayNumber: {
        fontSize: 14,
        color: theme.colors.text,
    },
    todayContainer: {
        backgroundColor: theme.colors.primary,
    },
    todayText: {
        color: 'white',
    }
});
