import React from 'react';
import { StyleSheet, View, Text, TouchableOpacity, Dimensions, Animated } from 'react-native';
import { theme } from '../styles/theme';

const WEEKDAYS = ['Lun', 'Mar', 'Mer', 'Jeu', 'Ven', 'Sam', 'Dim'];

export default function Calendar({ availabilities, currentMonthDate, onDateClick, filterFn, loading }) {
    const year = currentMonthDate.getFullYear();
    const month = currentMonthDate.getMonth();

    const date = new Date(year, month, 1);
    let firstDay = date.getDay() - 1;
    if (firstDay === -1) firstDay = 6;

    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const prevMonthLastDay = new Date(year, month, 0).getDate();

    const days = [];

    const today = new Date();
    const todayDay = today.getDate();
    const todayMonth = today.getMonth();
    const todayYear = today.getFullYear();

    // Padding prev month
    for (let i = 0; i < firstDay; i++) {
        const d = prevMonthLastDay - (firstDay - 1 - i);
        const cellDate = new Date(year, month - 1, d);
        const isToday = d === todayDay && cellDate.getMonth() === todayMonth && cellDate.getFullYear() === todayYear;

        days.push({
            day: d,
            isOtherMonth: true,
            isToday,
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

        const isToday = d === todayDay && month === todayMonth && year === todayYear;

        // Data is pending if we are currently loading OR if we don't have this date in the availabilities object yet
        const isPending = loading || !availabilities || !availabilities[dateStr];
        const hasAvailability = !isPending && filterFn ? filterFn(dateStr) : false;

        days.push({
            day: d,
            isOtherMonth: false,
            isToday,
            isPending,
            hasAvailability,
            dateStr,
            key: `curr-${d}`
        });
    }

    // Next month padding
    const totalCellsSoFar = days.length;
    const remainingCells = totalCellsSoFar % 7 === 0 ? 0 : 7 - (totalCellsSoFar % 7);

    for (let i = 1; i <= remainingCells; i++) {
        const cellDate = new Date(year, month + 1, i);
        const isToday = i === todayDay && cellDate.getMonth() === todayMonth && cellDate.getFullYear() === todayYear;

        days.push({
            day: i,
            isOtherMonth: true,
            isToday,
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
    const isToday = item.isToday;
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
        if (item.isPending) {
            statusStyle = styles.statusPending;
        } else {
            statusStyle = item.hasAvailability ? styles.statusAvailable : styles.statusUnavailable;
        }
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
        justifyContent: 'center', // Center content
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
    statusPending: {
        backgroundColor: '#F0F0F0', // Grey placeholder
    },
    dayNumberContainer: {
        width: 28,
        height: 28,
        borderRadius: 14,
        alignItems: 'center',
        justifyContent: 'center',
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
