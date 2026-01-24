import React, { useState, useEffect, useRef } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, ScrollView, Switch, ActivityIndicator, Alert } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import CreationModal from '../components/Modals/CreationModal';
import LoginModal from '../components/Modals/LoginModal';
import AvailabilityModal from '../components/Modals/AvailabilityModal';
import BookingModal from '../components/Modals/BookingModal';
import { AlarmService } from '../services/alarmService';
import AuthBadge from '../components/AuthBadge';


const WEEKDAYS_SHORT = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

export default function TimeSlotsScreen({ navigation, route, openDrawer, user, alarms, matchedResults, onSaveAlarm, onDeleteAlarm, onToggleAlarm, onClearMatchedResult, onLogin, onLogout }) {


    const [createModalVisible, setCreateModalVisible] = useState(false);
    const [editingAlarm, setEditingAlarm] = useState(null);


    // Modals state for matched results
    const [availModalVisible, setAvailModalVisible] = useState(false);
    const [selectedDate, setSelectedDate] = useState(null);
    const [selectedDayAvail, setSelectedDayAvail] = useState(null);
    const [selectedAlarm, setSelectedAlarm] = useState(null);
    const [bookingModalVisible, setBookingModalVisible] = useState(false);

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

    const toggleAlarm = (id) => {

        onToggleAlarm(id);
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
                                        value={alarm.activated}
                                        onValueChange={() => toggleAlarm(alarm.id)}
                                        trackColor={{ false: "#E0E0E0", true: "#1A1A1A" }}
                                    />
                                </View>

                                {matchedResults && matchedResults[alarm.name] && (
                                    <View style={styles.notifSection}>
                                        <View style={styles.notifHeader}>
                                            <Text style={styles.notifTitle}>Disponibilités trouvées :</Text>
                                            <TouchableOpacity onPress={() => onClearMatchedResult(alarm.name)}>
                                                <Text style={styles.notifClear}>Effacer</Text>
                                            </TouchableOpacity>
                                        </View>
                                        {Object.entries(matchedResults[alarm.name]).map(([date, dayPlan]) => {
                                            const slots = getAvailableSlots(dayPlan, alarm.slotDurations);
                                            if (slots.length === 0) return null;

                                            return (
                                                <TouchableOpacity
                                                    key={date}
                                                    style={styles.dateSection}
                                                    onPress={() => handleDateClick(date, dayPlan, alarm)}
                                                >

                                                    <Text style={styles.dateLabel}>{formatDate(date)} :</Text>
                                                    <View style={styles.slotsGrid}>
                                                        {slots.map((s, idx) => (
                                                            <View key={idx} style={styles.slotTag}>
                                                                <Text style={styles.slotText}>{s.time}</Text>
                                                            </View>
                                                        ))}
                                                    </View>
                                                </TouchableOpacity>
                                            );
                                        })}
                                    </View>
                                )}
                            </View>
                        ))}
                    </View>
                )}

                <View style={styles.buttonContainer}>
                    <TouchableOpacity
                        style={[styles.button, styles.primaryButton]}
                        onPress={openCreateModal}
                    >
                        <Text style={styles.primaryButtonText}>Créer un créneau</Text>

                    </TouchableOpacity>
                </View>
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


            {/* AuthBadge handles its own login modal internally */}


            <AvailabilityModal
                visible={availModalVisible}
                dateStr={selectedDate}
                dayAvail={selectedDayAvail}
                onClose={() => setAvailModalVisible(false)}
                onSlotClick={handleSlotClick}
                filterFn={(slot, playground) => {
                    if (!selectedAlarm) return true;
                    const durations = selectedAlarm.slotDurations || [3600, 5400, 7200];
                    return slot.prices.some(p => p.bookable && durations.includes(p.duration));
                }}
            />


            <BookingModal
                visible={bookingModalVisible}
                slotGroup={selectedSlot}
                onClose={() => setBookingModalVisible(false)}
            />
        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#FAFAFA',
    },
    header: {
        height: 56,
        flexDirection: 'row',
        alignItems: 'center',
        paddingHorizontal: 16,
        borderBottomWidth: 1,
        borderBottomColor: '#E0E0E0',
        backgroundColor: '#FFF',
    },
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
        flexDirection: 'row',
        backgroundColor: '#FFF',
        padding: 16,
        borderRadius: 12,
        marginBottom: 12,
        borderWidth: 1,
        borderColor: '#E8E8E8',
        flexWrap: 'wrap', // Allow notification data to wrap
    },
    alarmInfo: {
        flex: 1,
    },
    alarmHeader: {
        flexDirection: 'row',
        alignItems: 'center',
        justifyContent: 'space-between',
        marginBottom: 6,
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
        marginLeft: 16,
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
});
