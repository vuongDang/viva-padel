import React, { useState, useEffect, useCallback } from "react";
import { StyleSheet, View, Text, ScrollView, ActivityIndicator, TouchableOpacity } from "react-native";
import AsyncStorage from '@react-native-async-storage/async-storage';
import { SafeAreaView } from "react-native-safe-area-context";
import MonthNav from "../components/MonthNav";
import FilterBar from "../components/FilterBar";
import Calendar from "../components/Calendar";
import AvailabilityModal from "../components/Modals/AvailabilityModal";
import BookingModal from "../components/Modals/BookingModal";
import CreationModal from "../components/Modals/CreationModal";
import { matchesFilter } from "../utils/filterUtils";
import AuthBadge from "../components/AuthBadge";

import RefreshNote from "../components/RefreshNote";
import FloatingRefreshButton from "../components/FloatingRefreshButton";
import { theme } from '../styles/theme';







export default function CalendarScreen({
    navigation,
    openDrawer,
    availabilities,
    calendarTimestamp,
    loading,
    onRefresh,
    onInitialLoad,
    user,
    onLogin,
    onLogout,
    alarms,
    onSaveAlarm,
    onDeleteAlarm
}) {



    const [currentMonthDate, setCurrentMonthDate] = useState(new Date(2026, 0, 1));

    const [activeAlarmId, setActiveAlarmId] = useState("all");
    const [deleteMode, setDeleteMode] = useState(false);
    const [editMode, setEditMode] = useState(false);
    const [selectedAlarmToEdit, setSelectedAlarmToEdit] = useState(null);



    // Modals state
    const [availModalVisible, setAvailModalVisible] = useState(false);
    const [selectedDate, setSelectedDate] = useState(null);
    const [bookingModalVisible, setBookingModalVisible] = useState(false);
    const [selectedSlot, setSelectedSlot] = useState(null);
    const [createModalVisible, setCreateModalVisible] = useState(false);


    // Load active selection on mount
    useEffect(() => {
        onInitialLoad();

        const loadSavedTag = async () => {
            try {
                const savedTag = await AsyncStorage.getItem('last_selected_tag');
                if (savedTag) {
                    setActiveAlarmId(savedTag === "all" ? "all" : savedTag);
                }
            } catch (err) {
                console.error("Failed to load saved tag:", err);
            }
        };
        loadSavedTag();
    }, []);


    const handlePrevMonth = () => {
        setCurrentMonthDate(new Date(currentMonthDate.getFullYear(), currentMonthDate.getMonth() - 1, 1));
    };

    const handleNextMonth = () => {
        setCurrentMonthDate(new Date(currentMonthDate.getFullYear(), currentMonthDate.getMonth() + 1, 1));
    };

    useEffect(() => {
        const unsubscribe = navigation.addListener('blur', async () => {
            try {
                // Save the current active tag when leaving the screen
                await AsyncStorage.setItem('last_selected_tag', activeAlarmId.toString());
            } catch (err) {
                console.error("Failed to save tag on blur:", err);
            }
        });

        return unsubscribe;
    }, [navigation, activeAlarmId]);

    const handleSaveAlarmInternal = async (newAlarm) => {
        const savedAlarm = await onSaveAlarm(newAlarm);
        if (savedAlarm && savedAlarm.id) {
            setActiveAlarmId(savedAlarm.id);
        }
        setCreateModalVisible(false);
        setSelectedAlarmToEdit(null);
        setEditMode(false);
        setDeleteMode(false);
    };






    const handleAlarmSelect = async (id) => {
        if (id === "all") {
            setActiveAlarmId("all");
            return;
        }

        if (deleteMode) {
            onDeleteAlarm(id);
            if (activeAlarmId === id) {
                setActiveAlarmId("all");
            }
        } else if (editMode) {
            const alarm = alarms.find(a => a.id === id);
            if (alarm) {
                setSelectedAlarmToEdit(alarm);
                setCreateModalVisible(true);
            }
        } else {
            setActiveAlarmId(id);
        }
    };

    const toggleDeleteMode = () => {
        setDeleteMode(!deleteMode);
        if (!deleteMode) setEditMode(false);
    };

    const toggleEditMode = () => {
        setEditMode(!editMode);
        if (!editMode) setDeleteMode(false);
    };



    const checkAvailability = useCallback(
        (dateStr) => {
            const dayAvail = availabilities[dateStr];
            if (dayAvail && dayAvail["hydra:member"]) {
                return dayAvail["hydra:member"].some((playground) =>
                    playground.activities.some((activity) =>
                        activity.slots.some((slot) =>
                            matchesFilter(slot, playground, dateStr, activeAlarmId, alarms),
                        ),
                    ),
                );
            }
            return false;
        },
        [availabilities, activeAlarmId, alarms],
    );


    const onDateClick = (dateStr) => {
        if (checkAvailability(dateStr)) {
            setSelectedDate(dateStr);
            setAvailModalVisible(true);
        }
    };

    const onSlotClick = (slotGroup) => {
        setSelectedSlot(slotGroup);
        setBookingModalVisible(true);
    };

    return (
        <SafeAreaView style={styles.container}>
            <View style={styles.header}>
                <TouchableOpacity style={styles.menuButton} onPress={openDrawer}>
                    <Text style={styles.menuIcon}>☰</Text>
                </TouchableOpacity>
                <Text style={styles.headerTitle}>Calendrier</Text>
                <View style={styles.headerSpacer} />
                <AuthBadge user={user} onLogin={onLogin} onLogout={onLogout} />
            </View>

            <View style={styles.filterSection}>
                <FilterBar
                    filters={alarms}
                    activeFilterId={activeAlarmId}
                    onSelectFilter={handleAlarmSelect}
                    onDeleteMode={toggleDeleteMode}
                    isDeleteMode={deleteMode}
                    onEditMode={toggleEditMode}
                    isEditMode={editMode}
                    onCreateFilter={() => {
                        setSelectedAlarmToEdit(null);
                        setCreateModalVisible(true);
                    }}
                />
            </View>

            <View style={styles.divider} />

            <ScrollView contentContainerStyle={styles.content}>
                <MonthNav currentDate={currentMonthDate} onPrevMonth={handlePrevMonth} onNextMonth={handleNextMonth} />
                <Calendar availabilities={availabilities} currentMonthDate={currentMonthDate} onDateClick={onDateClick} filterFn={checkAvailability} loading={loading} />
                <RefreshNote timestamp={calendarTimestamp} />
            </ScrollView>

            <View style={styles.floatingButtonContainer}>
                <FloatingRefreshButton onPress={onRefresh} loading={loading} style={styles.calendarRefreshButton} />
            </View>






            <AvailabilityModal
                visible={availModalVisible}
                dateStr={selectedDate}
                dayAvail={selectedDate ? availabilities[selectedDate] : null}
                onClose={() => setAvailModalVisible(false)}
                onSlotClick={onSlotClick}
                filterFn={useCallback((slot, playground) => matchesFilter(slot, playground, selectedDate, activeAlarmId, alarms), [selectedDate, activeAlarmId, alarms])}
            />


            <BookingModal visible={bookingModalVisible} slotGroup={selectedSlot} onClose={() => setBookingModalVisible(false)} />
            <CreationModal
                visible={createModalVisible}
                onClose={() => {
                    setCreateModalVisible(false);
                    setSelectedAlarmToEdit(null);
                }}
                onCreate={handleSaveAlarmInternal}
                onDelete={(id) => {
                    onDeleteAlarm(id);
                    setCreateModalVisible(false);
                    setSelectedAlarmToEdit(null);
                    setEditMode(false);
                    setDeleteMode(false);
                }}

                initialData={selectedAlarmToEdit}
            />



        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: theme.colors.background
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
    headerSpacer: {
        flex: 1,
    },
    disabledButton: {
        opacity: 0.6,
    },
    todayBg: {
        backgroundColor: theme.colors.todayBg,
    },
    floatingButtonContainer: theme.styles.floatingButtonContainer,
    calendarRefreshButton: {
        minWidth: 140,
    },






    content: {
        padding: 20,
        paddingTop: 10,
        paddingBottom: 60
    },
    filterSection: {
        backgroundColor: '#FFF',
        paddingBottom: 4,
        borderBottomWidth: 1,
        borderBottomColor: '#F0F0F0',
    },
    loadingOverlay: {
        ...StyleSheet.absoluteFillObject,
        backgroundColor: 'rgba(255,255,255,0.7)',
        justifyContent: 'center',
        alignItems: 'center',
        zIndex: 1000,
    },
    divider: {
        height: 12,
        backgroundColor: '#FAFAFA', // Match screen background
    },


    footerNote: {
        marginTop: 20,
        textAlign: 'center',
        fontSize: 12,
        color: '#999',
        fontStyle: 'italic'
    },
});
