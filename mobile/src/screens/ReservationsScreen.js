import React, { useState, useEffect, useCallback } from "react";
import { StyleSheet, View, Text, ScrollView, ActivityIndicator, TouchableOpacity } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import MonthNav from "../components/MonthNav";
import FilterBar from "../components/FilterBar";
import Calendar from "../components/Calendar";
import AvailabilityModal from "../components/Modals/AvailabilityModal";
import BookingModal from "../components/Modals/BookingModal";
import CreationModal from "../components/Modals/CreationModal";
import { matchesFilter } from "../utils/filterUtils";
import { FilterService } from "../services/filterService";

export default function ReservationsScreen({
    navigation,
    openDrawer,
    availabilities,
    loading,
    onRefresh,
    onInitialLoad
}) {
    const [currentMonthDate, setCurrentMonthDate] = useState(new Date(2026, 0, 1));

    const evening_week_filter = {
        id: "default-filter-evenings",
        name: "Soirs semaine int",
        types: { indoor: true, outdoor: false },
        weekdays: [0, 1, 2, 3, 4],
        startTime: "17:30",
        endTime: "21:00",
    };

    const lunch_week_filter = {
        id: "default-filter-lunch",
        name: "Midis semaine int",
        types: { indoor: true, outdoor: false },
        weekdays: [0, 1, 2, 3, 4],
        startTime: "11:45",
        endTime: "12:30",
    };

    const [filters, setFilters] = useState([]);
    const [activeFilterId, setActiveFilterId] = useState("all");
    const [deleteMode, setDeleteMode] = useState(false);

    // Modals state
    const [availModalVisible, setAvailModalVisible] = useState(false);
    const [selectedDate, setSelectedDate] = useState(null);
    const [bookingModalVisible, setBookingModalVisible] = useState(false);
    const [selectedSlot, setSelectedSlot] = useState(null);
    const [createFilterVisible, setCreateFilterVisible] = useState(false);

    // Load filters and active selection on mount
    useEffect(() => {
        const loadInitialData = async () => {
            const savedFilters = await FilterService.loadFilters();
            const lastActiveId = await FilterService.loadActiveFilterId();

            if (savedFilters && savedFilters.length > 0) {
                setFilters(savedFilters);
            } else {
                setFilters([evening_week_filter, lunch_week_filter]);
            }

            if (lastActiveId) {
                setActiveFilterId(lastActiveId);
            }
        };
        loadInitialData();
        onInitialLoad();
    }, []);

    const handlePrevMonth = () => {
        setCurrentMonthDate(new Date(currentMonthDate.getFullYear(), currentMonthDate.getMonth() - 1, 1));
    };

    const handleNextMonth = () => {
        setCurrentMonthDate(new Date(currentMonthDate.getFullYear(), currentMonthDate.getMonth() + 1, 1));
    };

    const handleCreateFilter = async (newFilter) => {
        const id = "filter-" + Date.now().toString();
        const updatedFilters = [...filters, { ...newFilter, id }];
        setFilters(updatedFilters);
        await FilterService.saveFilters(updatedFilters);
        setCreateFilterVisible(false);
        setActiveFilterId(id);
        await FilterService.saveActiveFilterId(id);
    };

    const handleFilterSelect = async (id) => {
        if (deleteMode && id !== "all") {
            const updatedFilters = filters.filter((f) => f.id !== id);
            setFilters(updatedFilters);
            await FilterService.saveFilters(updatedFilters);
            if (activeFilterId === id) {
                setActiveFilterId("all");
                await FilterService.saveActiveFilterId("all");
            }
        } else {
            setActiveFilterId(id);
            await FilterService.saveActiveFilterId(id);
        }
    };

    const checkAvailability = useCallback(
        (dateStr) => {
            const dayAvail = availabilities[dateStr];
            if (dayAvail && dayAvail["hydra:member"]) {
                return dayAvail["hydra:member"].some((playground) =>
                    playground.activities.some((activity) =>
                        activity.slots.some((slot) =>
                            matchesFilter(slot, playground, dateStr, activeFilterId, filters),
                        ),
                    ),
                );
            }
            return false;
        },
        [availabilities, activeFilterId, filters],
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
                <Text style={styles.headerTitle}>Réservations</Text>
                <View style={styles.headerSpacer} />
                <TouchableOpacity style={styles.refreshButton} onPress={onRefresh} disabled={loading}>
                    {loading ? <ActivityIndicator size="small" color="#333" /> : <Text style={styles.refreshIcon}>↻</Text>}
                </TouchableOpacity>
            </View>

            <FilterBar
                filters={filters}
                activeFilterId={activeFilterId}
                onSelectFilter={handleFilterSelect}
                onDeleteMode={() => setDeleteMode(!deleteMode)}
                isDeleteMode={deleteMode}
                onCreateFilter={() => setCreateFilterVisible(true)}
            />

            <MonthNav currentDate={currentMonthDate} onPrevMonth={handlePrevMonth} onNextMonth={handleNextMonth} />

            <ScrollView contentContainerStyle={styles.content}>
                <Calendar availabilities={availabilities} currentMonthDate={currentMonthDate} onDateClick={onDateClick} filterFn={checkAvailability} />
                <Text style={styles.footerNote}>Les disponibilités sont rafraîchies toutes les 30 min.</Text>
            </ScrollView>

            <AvailabilityModal
                visible={availModalVisible}
                dateStr={selectedDate}
                dayAvail={selectedDate ? availabilities[selectedDate] : null}
                onClose={() => setAvailModalVisible(false)}
                onSlotClick={onSlotClick}
                filterFn={useCallback((slot, playground) => matchesFilter(slot, playground, selectedDate, activeFilterId, filters), [selectedDate, activeFilterId, filters])}
            />

            <BookingModal visible={bookingModalVisible} slotGroup={selectedSlot} onClose={() => setBookingModalVisible(false)} />
            <CreationModal
                visible={createFilterVisible}
                onClose={() => setCreateFilterVisible(false)}
                onCreate={handleCreateFilter}
                mode="filter"
            />
        </SafeAreaView>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#FAFAFA'
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
    headerSpacer: {
        flex: 1,
    },
    refreshButton: {
        width: 40,
        height: 40,
        justifyContent: 'center',
        alignItems: 'center',
    },
    refreshIcon: {
        fontSize: 20,
        color: '#333'
    },
    content: {
        padding: 20,
        paddingBottom: 60
    },
    footerNote: {
        marginTop: 20,
        textAlign: 'center',
        fontSize: 12,
        color: '#999',
        fontStyle: 'italic'
    },
});
