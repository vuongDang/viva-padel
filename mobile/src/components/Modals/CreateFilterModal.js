import React, { useState } from "react";
import {
  StyleSheet,
  View,
  Text,
  Modal,
  TouchableOpacity,
  TextInput,
  ScrollView,
  Alert,
} from "react-native";
import { theme } from "../../styles/theme";
import { Slider } from "@miblanchard/react-native-slider";

const WEEKDAYS = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

// Helper to convert "HH:MM" string to total minutes from midnight
const timeToMinutes = (timeStr) => {
  if (!timeStr || !timeStr.includes(":")) return 0;
  const [hours, minutes] = timeStr.split(":").map(Number);
  return hours * 60 + minutes;
};

// Helper to convert total minutes to "HH:MM" string format
const minutesToTime = (totalMinutes) => {
  if (typeof totalMinutes !== "number") return "00:00";
  const hours = Math.floor(totalMinutes / 60);
  const minutes = Math.round(totalMinutes % 60);
  return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(
    2,
    "0",
  )}`;
};

export default function CreateFilterModal({ visible, onClose, onCreate }) {
  const [name, setName] = useState("");
  const [indoor, setIndoor] = useState(true);
  const [outdoor, setOutdoor] = useState(true);
  const [selectedDays, setSelectedDays] = useState([0, 1, 2, 3, 4, 5, 6]);

  // Use a single state for the time range in minutes
  const [timeRange, setTimeRange] = useState([
    timeToMinutes("08:00"),
    timeToMinutes("22:00"),
  ]);

  const toggleDay = (index) => {
    if (selectedDays.includes(index)) {
      setSelectedDays(selectedDays.filter((d) => d !== index));
    } else {
      setSelectedDays([...selectedDays, index]);
    }
  };

  const handleSubmit = () => {
    if (!name.trim()) {
      Alert.alert("Incomplete Form", "Please enter a name for the filter.");
      return; // Stop the submission
    }

    const newFilter = {
      name: name.trim(),
      types: { indoor, outdoor },
      weekdays: selectedDays,
      startTime: minutesToTime(timeRange[0]), // Convert back to string
      endTime: minutesToTime(timeRange[1]), // Convert back to string
    };
    onCreate(newFilter);
    // Reset form to default values
    setName("");
    setIndoor(true);
    setOutdoor(true);
    setSelectedDays([0, 1, 2, 3, 4, 5, 6]);
    setTimeRange([timeToMinutes("08:00"), timeToMinutes("22:00")]);
  };

  return (
    <Modal
      animationType="slide"
      transparent={true}
      visible={visible}
      onRequestClose={onClose}
    >
      <TouchableOpacity
        style={styles.overlay}
        activeOpacity={1}
        onPress={onClose}
      >
        <TouchableOpacity
          activeOpacity={1}
          style={styles.content}
          onPress={(e) => e.stopPropagation()}
        >
          <View style={styles.headerRow}>
            <Text style={styles.title}>Créer un filtre</Text>
            <TouchableOpacity onPress={onClose}>
              <Text style={styles.closeText}>&times;</Text>
            </TouchableOpacity>
          </View>

          <ScrollView showsVerticalScrollIndicator={false}>
            <View style={styles.formGroup}>
              <Text style={styles.label}>Nom du filtre</Text>
              <TextInput
                style={styles.input}
                placeholder="Ex: Soirée Semaine"
                value={name}
                onChangeText={setName}
              />
            </View>

            <View style={styles.formGroup}>
              <Text style={styles.label}>Type de terrain</Text>
              <View style={styles.checkboxRow}>
                <TouchableOpacity
                  style={styles.checkboxItem}
                  onPress={() => setIndoor(!indoor)}
                >
                  <View
                    style={[styles.checkbox, indoor && styles.checkboxChecked]}
                  />
                  <Text>Intérieur</Text>
                </TouchableOpacity>
                <TouchableOpacity
                  style={styles.checkboxItem}
                  onPress={() => setOutdoor(!outdoor)}
                >
                  <View
                    style={[styles.checkbox, outdoor && styles.checkboxChecked]}
                  />
                  <Text>Extérieur</Text>
                </TouchableOpacity>
              </View>
            </View>

            <View style={styles.formGroup}>
              <Text style={styles.label}>Jours de la semaine</Text>
              <View style={styles.weekdaysRow}>
                {WEEKDAYS.map((day, i) => (
                  <TouchableOpacity
                    key={i}
                    style={[
                      styles.weekdayChip,
                      selectedDays.includes(i) && styles.weekdayChipActive,
                    ]}
                    onPress={() => toggleDay(i)}
                  >
                    <Text
                      style={[
                        styles.weekdayText,
                        selectedDays.includes(i) && styles.weekdayTextActive,
                      ]}
                    >
                      {day}
                    </Text>
                  </TouchableOpacity>
                ))}
              </View>
            </View>

            <View style={styles.formGroup}>
              <Text style={styles.label}>Début de créneau entre:</Text>
              <Text style={styles.timeLabel}>{`${minutesToTime(
                timeRange[0],
              )} - ${minutesToTime(timeRange[1])}`}</Text>
              <Slider
                value={timeRange}
                onValueChange={(value) => setTimeRange(value)}
                minimumValue={9 * 60}
                maximumValue={22 * 60}
                step={15}
                containerStyle={styles.sliderContainer}
              />
            </View>

            <TouchableOpacity style={styles.submitBtn} onPress={handleSubmit}>
              <Text style={styles.submitBtnText}>Enregistrer</Text>
            </TouchableOpacity>
          </ScrollView>
        </TouchableOpacity>
      </TouchableOpacity>
    </Modal>
  );
}

const styles = StyleSheet.create({
  overlay: {
    flex: 1,
    backgroundColor: "rgba(0,0,0,0.5)",
    justifyContent: "flex-end",
  },
  content: {
    backgroundColor: "white",
    borderTopLeftRadius: 16,
    borderTopRightRadius: 16,
    padding: 24,
    maxHeight: "80%",
  },
  headerRow: {
    flexDirection: "row",
    justifyContent: "space-between",
    marginBottom: 20,
  },
  title: {
    fontSize: 20,
    fontWeight: "600",
  },
  closeText: {
    fontSize: 28,
    color: "#70757a",
    marginTop: -5,
  },
  formGroup: {
    marginBottom: 20,
  },
  label: {
    fontWeight: "600",
    marginBottom: 8,
    fontSize: 16,
  },
  input: {
    borderWidth: 1,
    borderColor: theme.colors.border,
    borderRadius: 8,
    padding: 12,
    fontSize: 16,
  },
  checkboxRow: {
    flexDirection: "row",
    gap: 20,
  },
  checkboxItem: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
  },
  checkbox: {
    width: 20,
    height: 20,
    borderWidth: 2,
    borderColor: "#70757a",
    borderRadius: 4,
  },
  checkboxChecked: {
    backgroundColor: theme.colors.primary,
    borderColor: theme.colors.primary,
  },
  weekdaysRow: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 8,
  },
  weekdayChip: {
    paddingVertical: 6,
    paddingHorizontal: 10,
    borderRadius: 4,
    borderWidth: 1,
    borderColor: theme.colors.border,
  },
  weekdayChipActive: {
    backgroundColor: theme.colors.primary,
    borderColor: theme.colors.primary,
  },
  weekdayText: {
    fontSize: 12,
    color: theme.colors.text,
  },
  weekdayTextActive: {
    color: "white",
  },
  timeLabel: {
    textAlign: "center",
    fontSize: 18,
    fontWeight: "bold",
    marginVertical: 10,
    color: theme.colors.primary,
  },
  sliderContainer: {
    marginHorizontal: 10,
  },
  submitBtn: {
    backgroundColor: theme.colors.primary,
    padding: 16,
    borderRadius: 8,
    alignItems: "center",
    marginTop: 10,
  },
  submitBtnText: {
    color: "white",
    fontSize: 16,
    fontWeight: "600",
  },
});
