import React, { useState, useEffect, useRef, useMemo } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, ScrollView, Switch, ActivityIndicator, Alert } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import CreationModal from '../components/Modals/CreationModal';
import LoginModal from '../components/Modals/LoginModal';
import AvailabilityModal from '../components/Modals/AvailabilityModal';
import BookingModal from '../components/Modals/BookingModal';
import { AlarmService } from '../services/alarmService';
import { matchesFilter } from '../utils/filterUtils';
import AuthBadge from '../components/AuthBadge';
import RefreshNote from '../components/RefreshNote';
import FloatingRefreshButton from '../components/FloatingRefreshButton';
import NotificationActivationModal from '../components/Modals/NotificationActivationModal';
import { theme } from '../styles/theme';







const WEEKDAYS_SHORT = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

export default function TimeSlotsScreen({ navigation, route, openDrawer, user, alarms, availabilities, filteredMatches, calendarTimestamp, matchedResults, onSaveAlarm, onDeleteAlarm, onToggleAlarm, onClearMatchedResult, onSync, onLogin, onLogout, onRefresh, loading }) {






    const [createModalVisible, setCreateModalVisible] = useState(false);
    const [editingAlarm, setEditingAlarm] = useState(null);


    const [availModalVisible, setAvailModalVisible] = useState(false);
    const [selectedDate, setSelectedDate] = useState(null);
    const [selectedDayAvail, setSelectedDayAvail] = useState(null);
    const [selectedAlarm, setSelectedAlarm] = useState(null);
    const [bookingModalVisible, setBookingModalVisible] = useState(false);
    const [notifModalVisible, setNotifModalVisible] = useState(false);

    // Logic for hiding results (UI only, default is visible)
    const [hiddenAlarms, setHiddenAlarms] = useState(new Set());
    const [expandedAlarms, setExpandedAlarms] = useState(new Set());


    const [selectedSlot, setSelectedSlot] = useState(null);


    const formatDate = (dateStr) => {
        const date = new Date(dateStr);
        return date.toLocaleDateString('fr-FR', { weekday: 'short', day: 'numeric', month: 'short' });
    };

    const getAvailableSlots = (dayPlanning, allowedDurations) => {
        const slots = [];
        if (!dayPlanning.courts) return slots;

        const durations = allowedDurations || [3600, 5400, 7200];

        dayPlanning.courts.forEach(court => {
            if (!court.slots) return;
            court.slots.forEach(slot => {
                // Check if any price is bookable and duration matches
                if (slot.prices && slot.prices.some(p => p.bookable && durations.includes(p.duration))) {
                    slots.push({
                        time: slot.start_at,
                        court: court.name,
                        isIndoor: court.indoor
                    });
                }
            });
        });


        // Sort by time
        return slots.sort((a, b) => a.time.localeCompare(b.time));
    };

    const formatDays = (days) => {
        if (!days || days.length === 0) return "";
        if (days.length === 7) return "Tous les jours";
        return days.map(d => WEEKDAYS_SHORT[d]).join(', ');
    };

    const openEditModal = (alarm) => {
        setEditingAlarm(alarm);
        setCreateModalVisible(true);
    };

    const openCreateModal = () => {
        setEditingAlarm(null);
        setCreateModalVisible(true);
    };

    const toggleVisibility = (id) => {
        setHiddenAlarms(prev => {
            const next = new Set(prev);
            if (next.has(id)) next.delete(id);
            else next.add(id);
            return next;
        });
    };

    const toggleExpansion = (id) => {
        setExpandedAlarms(prev => {
            const next = new Set(prev);
            if (next.has(id)) next.delete(id);
            else next.add(id);
            return next;
        });
    };


    const handleDateClick = (date, dayPlan, alarm) => {
        setSelectedDate(date);
        setSelectedDayAvail(dayPlan);
        setSelectedAlarm(alarm);
        setAvailModalVisible(true);
    };


    const handleSlotClick = (slotGroup) => {
        setSelectedSlot(slotGroup);
        setBookingModalVisible(true);
    };

    return (
        <SafeAreaView style={styles.container}>
            <View style={styles.header}>
                <TouchableOpacity style={styles.menuButton} onPress={openDrawer}>
                    <Text style={styles.menuIcon}>☰</Text>
                </TouchableOpacity>
                <Text style={styles.headerTitle}>Mes créneaux</Text>
                <View style={{ flex: 1 }} />
                <AuthBadge user={user} onLogin={onLogin} onLogout={onLogout} />
            </View>


            <ScrollView contentContainerStyle={styles.scrollContent}>
                <TouchableOpacity
                    style={[styles.button, styles.primaryButton, { marginBottom: 16 }]}
                    onPress={openCreateModal}
                >
                    <Text style={styles.primaryButtonText}>Créer un créneau</Text>
                </TouchableOpacity>

                {alarms.length === 0 ? (
                    <View style={styles.emptyContent}>
                        <Text style={styles.title}>Aucun créneau configuré</Text>
                        <Text style={styles.subtitle}>Configurez des alertes pour être notifié des disponibilités.</Text>

                    </View>
                ) : (
                    <View style={styles.alarmList}>
                        {alarms.map((alarm) => (


                            <View key={alarm.id} style={styles.alarmCard}>
                                <TouchableOpacity

                                    style={styles.deleteButton}
                                    onPress={() => onDeleteAlarm(alarm.id)}
                                >
                                    <View style={styles.deleteIconContainer}>
                                        <Text style={styles.deleteIconText}>×</Text>
                                    </View>
                                </TouchableOpacity>

                                <View style={styles.alarmMainContent}>
                                    <TouchableOpacity
                                        style={styles.alarmInfo}
                                        onPress={() => openEditModal(alarm)}
                                    >
                                        <View style={styles.alarmHeader}>
                                            <Text style={styles.alarmName}>{alarm.name}</Text>
                                        </View>
                                        <Text style={styles.alarmDetails}>
                                            {formatDays(alarm.weekdays)}
                                        </Text>
                                        <Text style={styles.alarmDetails}>
                                            {alarm.startTime} — {alarm.endTime}
                                        </Text>
                                    </TouchableOpacity>

                                    <View style={styles.alarmRightActions}>
                                        <Switch
                                            value={!hiddenAlarms.has(alarm.id)}
                                            onValueChange={() => toggleVisibility(alarm.id)}
                                            trackColor={{ false: "#E0E0E0", true: "#1A1A1A" }}
                                        />
                                    </View>
                                </View>


                                {/* Local Availability Matches */}
                                {!hiddenAlarms.has(alarm.id) && (() => {
                                    const matchesForCurrent = filteredMatches[alarm.id] || {};
                                    const matchingDays = Object.entries(matchesForCurrent);

                                    if (matchingDays.length === 0) {
                                        return (
                                            <View style={styles.localMatchesSection}>
                                                <Text style={styles.emptyMatchesText}>Aucune disponibilité trouvée</Text>
                                            </View>
                                        );
                                    }

                                    const isExpanded = expandedAlarms.has(alarm.id);

                                    return (
                                        <View style={styles.localMatchesSection}>
                                            <View style={[
                                                styles.daysContainer,
                                                !isExpanded && matchingDays.length > 5 && styles.collapsedContainer
                                            ]}>
                                                {matchingDays.map(([dateStr, dayAvail]) => (
                                                    <TouchableOpacity
                                                        key={dateStr}
                                                        style={[styles.dayChip, styles.localDayChip]}
                                                        onPress={() => handleDateClick(dateStr, dayAvail, alarm)}
                                                    >
                                                        <Text style={[styles.dayText, styles.localDayText]}>{formatDate(dateStr)}</Text>
                                                    </TouchableOpacity>
                                                ))}
                                            </View>
                                            {matchingDays.length > 5 && (
                                                <TouchableOpacity
                                                    style={styles.showMoreButton}
                                                    onPress={() => toggleExpansion(alarm.id)}
                                                >
                                                    <Text style={styles.showMoreText}>
                                                        {isExpanded ? "Voir moins" : `Voir plus (${matchingDays.length - 5} de plus)`}
                                                    </Text>
                                                </TouchableOpacity>
                                            )}
                                        </View>
                                    );
                                })()}


                                {!hiddenAlarms.has(alarm.id) && matchedResults && matchedResults[alarm.name] && (
                                    <View style={styles.notifSection}>
                                        <View style={styles.notifHeader}>
                                            <Text style={styles.notifTitle}>Disponibilités trouvées :</Text>
                                            <TouchableOpacity onPress={() => onClearMatchedResult(alarm.name)}>
                                                <Text style={styles.notifClear}>Effacer</Text>
                                            </TouchableOpacity>
                                        </View>
                                        <View style={[
                                            styles.daysContainer,
                                            !expandedAlarms.has(alarm.name) && Object.keys(matchedResults[alarm.name]).length > 5 && styles.collapsedContainer
                                        ]}>
                                            {Object.entries(matchedResults[alarm.name]).map(([date, dayPlan]) => (
                                                <TouchableOpacity
                                                    key={date}
                                                    style={styles.dayChip}
                                                    onPress={() => handleDateClick(date, dayPlan, alarm)}
                                                >
                                                    <Text style={styles.dayText}>{formatDate(date)}</Text>
                                                </TouchableOpacity>
                                            ))}
                                        </View>
                                        {Object.keys(matchedResults[alarm.name]).length > 5 && (
                                            <TouchableOpacity
                                                style={styles.showMoreButton}
                                                onPress={() => toggleExpansion(alarm.name)}
                                            >
                                                <Text style={styles.showMoreText}>
                                                    {expandedAlarms.has(alarm.name) ? "Voir moins" : `Voir plus (${Object.keys(matchedResults[alarm.name]).length - 5} de plus)`}
                                                </Text>
                                            </TouchableOpacity>
                                        )}

                                    </View>
                                )}


                            </View>
                        ))}
                    </View>
                )}

                {alarms.length > 0 && (
                    <RefreshNote timestamp={calendarTimestamp} style={styles.footerNoteAdjust} />
                )}
            </ScrollView>




            <CreationModal
                visible={createModalVisible}
                onClose={() => {
                    setCreateModalVisible(false);
                    setEditingAlarm(null);
                }}
                onCreate={(alarm) => {
                    onSaveAlarm(alarm);
                    setCreateModalVisible(false);
                }}
                onDelete={onDeleteAlarm}
                mode="alarm"
                initialData={editingAlarm}
            />



            <AvailabilityModal
                visible={availModalVisible}
                dateStr={selectedDate}
                dayAvail={selectedDayAvail}
                onClose={() => setAvailModalVisible(false)}
                onSlotClick={handleSlotClick}
                filterFn={(slot, playground) => {
                    if (!selectedAlarm) return true;
                    return matchesFilter(slot, playground, selectedDate, selectedAlarm.id, alarms);
                }}
            />



            <BookingModal
                visible={bookingModalVisible}
                slotGroup={selectedSlot}
                onClose={() => setBookingModalVisible(false)}
            />

            {alarms.length > 0 && (
                <View style={styles.floatingButtonsContainer}>
                    <FloatingRefreshButton
                        onPress={onRefresh}
                        loading={loading}
                        style={styles.flexButton}
                    />

                    <TouchableOpacity
                        style={styles.notifFloatingButton}
                        onPress={() => setNotifModalVisible(true)}
                    >
                        <Text
                            style={styles.notifFloatingButtonText}
                        >
                            Activer les notifications
                        </Text>
                    </TouchableOpacity>
                </View>
            )}



            <NotificationActivationModal
                visible={notifModalVisible}
                onClose={() => setNotifModalVisible(false)}
                alarms={alarms}
                onToggleAlarm={onToggleAlarm}
                onSync={onSync}
            />
        </SafeAreaView >

    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: theme.colors.background,
    },
    header: theme.styles.header,

    menuButton: {
        width: 40,
        height: 40,
        justifyContent: 'center',
    },
    menuIcon: {
        fontSize: 22,
        color: '#333',
    },
    headerTitle: {
        fontSize: 18,
        fontWeight: '600',
        color: '#1A1A1A',
        marginLeft: 8,
    },
    scrollContent: {
        flexGrow: 1,
        padding: 24,
    },
    emptyContent: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
        paddingVertical: 60,
    },
    title: {
        fontSize: 18,
        fontWeight: '600',
        color: '#333',
        marginBottom: 8,
    },
    subtitle: {
        fontSize: 14,
        color: '#888',
        textAlign: 'center',
        lineHeight: 20,
        marginBottom: 32,
    },
    alarmList: {
        marginBottom: 32,
    },
    alarmCard: {
        backgroundColor: '#FFF',
        padding: 16,
        borderRadius: 12,
        marginBottom: 12,
        borderWidth: 1,
        borderColor: '#E8E8E8',
        position: 'relative', // For absolute delete button
    },
    alarmMainContent: {
        flexDirection: 'row',
        alignItems: 'center', // Center switch vertically relative to info
    },
    alarmInfo: {
        flex: 1,
    },
    alarmHeader: {
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'space-between',
        marginBottom: 4,
        paddingRight: 16,
    },
    alarmName: {
        fontSize: 16,
        fontWeight: '600',
        color: '#1A1A1A',
    },
    alarmDetails: {
        fontSize: 13,
        color: '#888',
        marginTop: 2,
    },
    alarmRightActions: {
        paddingLeft: 12,
        justifyContent: 'center',
    },
    deleteButton: {
        position: 'absolute',
        top: -8,
        right: -8,
        zIndex: 10,
    },
    deleteIconContainer: {
        backgroundColor: '#FFF',
        width: 24,
        height: 24,
        borderRadius: 12,
        borderWidth: 1,
        borderColor: '#E0E0E0',
        alignItems: 'center',
        justifyContent: 'center',
        shadowColor: "#000",
        shadowOffset: { width: 0, height: 1 },
        shadowOpacity: 0.1,
        shadowRadius: 1,
        elevation: 1,
    },
    deleteIconText: {
        fontSize: 18,
        color: '#999',
        fontWeight: '400',
        lineHeight: 20,
    },

    buttonContainer: {
        flexDirection: 'row',
        justifyContent: 'center',
        gap: 12,
        width: '100%',
        marginTop: 'auto',
    },
    button: {
        paddingHorizontal: 24,
        paddingVertical: 12,
        borderRadius: 8,
        minWidth: 120,
        alignItems: 'center',
        justifyContent: 'center',
    },
    primaryButton: {
        backgroundColor: '#1A1A1A',
    },
    primaryButtonText: {
        color: '#FFF',
        fontSize: 15,
        fontWeight: '600',
    },
    secondaryButton: {
        backgroundColor: 'transparent',
        borderWidth: 1,
        borderColor: '#E0E0E0',
    },
    secondaryButtonText: {
        color: '#333',
        fontSize: 15,
        fontWeight: '600',
    },
    disabledButton: {
        opacity: 0.5,
    },
    notifSection: {
        width: '100%',
        marginTop: 16,
        paddingTop: 16,
        borderTopWidth: 1,
        borderTopColor: '#F0F0F0',
    },
    notifHeader: {
        flexDirection: 'row',
        justifyContent: 'space-between',
        alignItems: 'center',
        marginBottom: 12,
    },
    notifTitle: {
        fontSize: 14,
        fontWeight: '700',
        color: '#34A853', // Green for positive availability
    },
    notifClear: {
        fontSize: 12,
        color: '#999',
    },
    dateSection: {
        marginBottom: 12,
    },
    dateLabel: {
        fontSize: 13,
        fontWeight: '600',
        color: '#555',
        marginBottom: 6,
    },
    slotsGrid: {
        flexDirection: 'row',
        flexWrap: 'wrap',
        gap: 6,
    },
    slotTag: {
        backgroundColor: '#E6F4EA',
        paddingHorizontal: 8,
        paddingVertical: 4,
        borderRadius: 4,
        borderWidth: 1,
        borderColor: '#CEEAD6',
    },
    slotText: {
        fontSize: 12,
        color: '#137333',
        fontWeight: '500',
    },
    localMatchesSection: {
        width: '100%',
        marginTop: 4,
    },
    daysContainer: {
        flexDirection: 'row',
        flexWrap: 'wrap',
        gap: 8,
        marginTop: 8,
    },
    dayChip: {
        backgroundColor: '#E6F4EA',
        paddingHorizontal: 10,
        paddingVertical: 5,
        borderRadius: 14,
        borderWidth: 1,
        borderColor: '#CEEAD6',
    },
    dayText: {
        fontSize: 12,
        color: '#137333',
        fontWeight: '600',
    },
    localDayChip: {
        backgroundColor: '#E8F0FE',
        borderColor: '#D2E3FC',
    },
    localDayText: {
        color: '#1A73E8',
    },
    emptyMatchesText: {
        fontSize: 12,
        color: '#999',
        fontStyle: 'italic',
        marginTop: 4,
    },
    footerNoteAdjust: {

        marginTop: 0,
        marginBottom: 20,
    },
    floatingButtonsContainer: theme.styles.floatingButtonContainer,
    notifFloatingButton: {
        backgroundColor: theme.colors.primary,
        borderRadius: 25,
        paddingHorizontal: 12,
        height: 52,
        justifyContent: 'center',
        alignItems: 'center',
        ...theme.shadows.medium,
        flex: 1,
        maxWidth: 220,
    },
    flexButton: {
        flex: 1,
        maxWidth: 220,
        height: 52,
    },
    notifFloatingButtonText: {
        fontSize: 14,
        fontWeight: '700',
        color: theme.colors.white,
        textAlign: 'center',
    },
    collapsedContainer: {
        maxHeight: 70, // Limits to ~2 lines of chips
        overflow: 'hidden',
    },
    showMoreButton: {
        marginTop: 8,
        paddingVertical: 4,
    },
    showMoreText: {
        fontSize: 12,
        color: '#1A73E8',
        fontWeight: '600',
    },



});