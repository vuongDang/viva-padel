import React, { useState, useEffect, useCallback } from "react";
import { StyleSheet, View, Text, ScrollView, Alert } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { theme } from "../styles/theme";
import MonthNav from "../components/MonthNav";
import FilterBar from "../components/FilterBar";
import Calendar from "../components/Calendar";
import AvailabilityModal from "../components/Modals/AvailabilityModal";
import BookingModal from "../components/Modals/BookingModal";
import CreateFilterModal from "../components/Modals/CreateFilterModal";
import { matchesFilter } from "../utils/filterUtils";

import mockData from "../data/mock_calendar.json";

export default function HomeScreen() {
  const [currentMonthDate, setCurrentMonthDate] = useState(
    new Date(2026, 0, 1),
  );
  const [availabilities, setAvailabilities] = useState({});

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

  const [filters, setFilters] = useState([
    evening_week_filter,
    lunch_week_filter,
  ]);
  const [activeFilterId, setActiveFilterId] = useState("all");
  const [deleteMode, setDeleteMode] = useState(false);

  // Modals state
  const [availModalVisible, setAvailModalVisible] = useState(false);
  const [selectedDate, setSelectedDate] = useState(null);
  const [bookingModalVisible, setBookingModalVisible] = useState(false);
  const [selectedSlot, setSelectedSlot] = useState(null);
  const [createFilterVisible, setCreateFilterVisible] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      // Determine API URL based on platform
      const baseUrl = "https://xoi-lap-xuong.com";
      const apiUrl = `${baseUrl}/viva-padel/calendar`;
      console.log(`Fetching from: ${apiUrl}`);

      try {
        const response = await fetch(apiUrl, {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            "CF-Access-Client-Id": process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID,
            "CF-Access-Client-Secret":
              process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET,
          },
        });
        const responseText = await response.text(); // Read body as text first

        if (!response.ok) {
          console.error(
            `HTTP error! Status: ${response.status}, Body: ${responseText}`,
          );
          Alert.alert(
            "Server Error",
            `The server responded with an error (Status: ${response.status}).`,
            [{ text: "OK" }],
          );
          return;
        }

        try {
          const data = JSON.parse(responseText); // Now, try to parse the text
          setAvailabilities(data.availabilities || {});
        } catch (jsonError) {
          console.error("Failed to parse JSON:", jsonError.message);
          console.error(
            "Raw response body that failed to parse:",
            responseText,
          );
          Alert.alert(
            "Invalid Response",
            "Received an invalid response from the server.",
            [{ text: "OK" }],
          );
        }
      } catch (networkError) {
        console.error("Network error fetching data:", networkError.message);
        Alert.alert(
          "Connection Error",
          "The server is not available. Please try again later.",
          [{ text: "OK" }],
        );
      }
    };

    fetchData();
  }, []);

  const handlePrevMonth = () => {
    setCurrentMonthDate(
      new Date(
        currentMonthDate.getFullYear(),
        currentMonthDate.getMonth() - 1,
        1,
      ),
    );
  };

  const handleNextMonth = () => {
    setCurrentMonthDate(
      new Date(
        currentMonthDate.getFullYear(),
        currentMonthDate.getMonth() + 1,
        1,
      ),
    );
  };

  const handleCreateFilter = (newFilter) => {
    const id = "filter-" + Date.now().toString();
    const filterWithId = {
      ...newFilter,
      id: id,
    };
    setFilters([...filters, filterWithId]);
    setCreateFilterVisible(false);
    setActiveFilterId(id);
  };

  const handleFilterSelect = (id) => {
    if (deleteMode && id !== "all") {
      setFilters(filters.filter((f) => f.id !== id));
      if (activeFilterId === id) setActiveFilterId("all");
    } else {
      setActiveFilterId(id);
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
    // Only open if there are availabilities? Frontend allows opening even if unavailable?
    // Frontend: div.onclick = () => showModal(dateStr) ONLY if isAvailable
    if (checkAvailability(dateStr)) {
      setSelectedDate(dateStr);
      setAvailModalVisible(true);
    } else {
      // Maybe show toast? Frontend does nothing if unavailable (no click handler)
    }
  };

  const onSlotClick = (slotGroup) => {
    setSelectedSlot(slotGroup);
    setBookingModalVisible(true);
    // Note: Availability modal stays open behind booking modal?
    // Frontend: Yes, booking modal opens on top.
  };

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.headerTitle}>Réservations de Padel</Text>
      </View>

      <FilterBar
        filters={filters}
        activeFilterId={activeFilterId}
        onSelectFilter={handleFilterSelect}
        onDeleteMode={() => setDeleteMode(!deleteMode)}
        isDeleteMode={deleteMode}
        onCreateFilter={() => setCreateFilterVisible(true)}
      />

      <MonthNav
        currentDate={currentMonthDate}
        onPrevMonth={handlePrevMonth}
        onNextMonth={handleNextMonth}
      />

      <ScrollView contentContainerStyle={styles.content}>
        <Calendar
          availabilities={availabilities}
          currentMonthDate={currentMonthDate}
          onDateClick={onDateClick}
          filterFn={checkAvailability}
        />
        <Text style={styles.footerNote}>
          Les disponibilités sont rafraîchies toutes les 30 min. Seuls les 3 prochains mois sont pris en compte.
        </Text>
      </ScrollView>

      {/* Modals */}
      <AvailabilityModal
        visible={availModalVisible}
        dateStr={selectedDate}
        dayAvail={selectedDate ? availabilities[selectedDate] : null}
        onClose={() => setAvailModalVisible(false)}
        onSlotClick={onSlotClick}
        filterFn={useCallback((slot, playground) =>
          matchesFilter(slot, playground, selectedDate, activeFilterId, filters),
          [selectedDate, activeFilterId, filters]
        )}
      />

      <BookingModal
        visible={bookingModalVisible}
        slotGroup={selectedSlot}
        onClose={() => setBookingModalVisible(false)}
      />

      <CreateFilterModal
        visible={createFilterVisible}
        onClose={() => setCreateFilterVisible(false)}
        onCreate={handleCreateFilter}
      />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: theme.colors.background,
  },
  header: {
    padding: theme.spacing.m,
    borderBottomWidth: 1,
    borderBottomColor: theme.colors.border,
    alignItems: "center",
    backgroundColor: theme.colors.background,
  },
  headerTitle: {
    color: theme.colors.primary,
    fontSize: theme.text.header.fontSize,
    fontWeight: theme.text.header.fontWeight,
  },
  content: {
    padding: theme.spacing.m,
    paddingBottom: 60,
  },
  footerNote: {
    marginTop: theme.spacing.xl,
    textAlign: "center",
    fontSize: 12,
    color: theme.colors.textSecondary || "#666",
    fontStyle: "italic",
    opacity: 0.8,
  },
});
