import React, { useState, useEffect } from "react";
import {
    StyleSheet,
    View,
    Text,
    Modal,
    TouchableOpacity,
    TextInput,
    ScrollView,
} from "react-native";
import { Slider } from "@miblanchard/react-native-slider";

const WEEKDAYS = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

const timeToMinutes = (timeStr) => {
    if (!timeStr || !timeStr.includes(":")) return 0;
    const [hours, minutes] = timeStr.split(":").map(Number);
    return hours * 60 + minutes;
};

const minutesToTime = (totalMinutes) => {
    if (typeof totalMinutes !== "number") return "00:00";
    const hours = Math.floor(totalMinutes / 60);
    const minutes = Math.round(totalMinutes % 60);
    return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}`;
};

/**
 * Shared Configuration Modal for both Alarms and Filters.
 * mode: 'alarm' | 'filter'
 */
export default function CreationModal({ visible, onClose, onCreate, onDelete, mode = 'filter', initialData = null }) {
    const isAlarm = mode === 'alarm';
    const isEditing = !!initialData;

    const [name, setName] = useState("");
    const [indoor, setIndoor] = useState(true);
    const [outdoor, setOutdoor] = useState(mode === 'filter');
    const [selectedDays, setSelectedDays] = useState([0, 1, 2, 3, 4, 5, 6]);
    const [timeRange, setTimeRange] = useState([
        timeToMinutes(isAlarm ? "17:30" : "08:00"),
        timeToMinutes("22:00"),
    ]);
    const [period, setPeriod] = useState(2);
    const [slotDurations, setSlotDurations] = useState([3600, 5400, 7200]);

    useEffect(() => {
        if (visible) {
            if (initialData) {
                setName(initialData.name || "");
                setIndoor(initialData.types?.indoor ?? true);
                setOutdoor(initialData.types?.outdoor ?? (mode === 'filter'));
                setSelectedDays(initialData.weekdays || [0, 1, 2, 3, 4, 5, 6]);
                setTimeRange([
                    timeToMinutes(initialData.startTime || (isAlarm ? "17:30" : "08:00")),
                    timeToMinutes(initialData.endTime || "22:00"),
                ]);
                setPeriod(initialData.period || 2);
                setSlotDurations(initialData.slotDurations || [3600, 5400, 7200]);
            } else {
                // Reset to defaults for new creation
                setName("");
                setIndoor(true);
                setOutdoor(mode === 'filter');
                setSelectedDays([0, 1, 2, 3, 4, 5, 6]);
                setTimeRange([
                    timeToMinutes(isAlarm ? "17:30" : "08:00"),
                    timeToMinutes("22:00"),
                ]);
                setPeriod(2);
                setSlotDurations([3600, 5400, 7200]);
            }
        }
    }, [visible, initialData, mode, isAlarm]);

    const toggleDuration = (duration) => {
        if (slotDurations.includes(duration)) {
            if (slotDurations.length > 1) {
                setSlotDurations(slotDurations.filter(d => d !== duration));
            }
        } else {
            setSlotDurations([...slotDurations, duration].sort((a, b) => a - b));
        }
    };

    const toggleDay = (index) => {
        if (selectedDays.includes(index)) {
            setSelectedDays(selectedDays.filter((d) => d !== index));
        } else {
            setSelectedDays([...selectedDays, index]);
        }
    };

    const handleSubmit = () => {
        const trimmedName = name.trim();
        const config = {
            name: trimmedName || (isAlarm ? "Mon Créneau" : "Mon Filtre"),

            types: { indoor, outdoor },
            weekdays: selectedDays,
            startTime: minutesToTime(timeRange[0]),
            endTime: minutesToTime(timeRange[1]),
            slotDurations: slotDurations,
        };

        if (isAlarm) {
            config.period = period;
            config.activated = true;
        }

        if (initialData?.id) {
            config.id = initialData.id;
        }

        onCreate(config);
        onClose();
    };

    return (
        <Modal
            animationType="slide"
            transparent={true}
            visible={visible}
            onRequestClose={onClose}
        >
            <View style={styles.overlay}>
                <TouchableOpacity style={styles.backdrop} activeOpacity={1} onPress={onClose} />
                <View style={styles.content}>
                    <View style={styles.header}>
                        <Text style={styles.title}>
                            {isEditing ? (isAlarm ? "Modifier le Créneau" : "Modifier le Filtre") : (isAlarm ? "Nouveau Créneau" : "Nouveau Filtre")}

                        </Text>
                        <TouchableOpacity onPress={onClose}>
                            <Text style={styles.closeIcon}>✕</Text>
                        </TouchableOpacity>
                    </View>

                    <ScrollView showsVerticalScrollIndicator={false} contentContainerStyle={styles.scrollContent}>
                        <View style={styles.formGroup}>
                            <Text style={styles.label}>Nom</Text>
                            <TextInput
                                style={styles.input}
                                placeholder={isAlarm ? "Ex: Soirée Semaine" : "Ex: Tous les soirs"}
                                value={name}
                                onChangeText={setName}
                                placeholderTextColor="#AAA"
                            />
                        </View>

                        <View style={styles.formGroup}>
                            <Text style={styles.label}>Type</Text>
                            <View style={styles.row}>
                                <TouchableOpacity
                                    style={[styles.chip, indoor && styles.chipActive]}
                                    onPress={() => setIndoor(!indoor)}
                                >
                                    <Text style={[styles.chipText, indoor && styles.chipTextActive]}>Intérieur</Text>
                                </TouchableOpacity>
                                <TouchableOpacity
                                    style={[styles.chip, outdoor && styles.chipActive]}
                                    onPress={() => setOutdoor(!outdoor)}
                                >
                                    <Text style={[styles.chipText, outdoor && styles.chipTextActive]}>Extérieur</Text>
                                </TouchableOpacity>
                            </View>
                        </View>

                        <View style={styles.formGroup}>
                            <Text style={styles.label}>Jours</Text>
                            <View style={styles.weekdaysRow}>
                                {WEEKDAYS.map((day, i) => (
                                    <TouchableOpacity
                                        key={i}
                                        style={[
                                            styles.dayCircle,
                                            selectedDays.includes(i) && styles.dayCircleActive,
                                        ]}
                                        onPress={() => toggleDay(i)}
                                    >
                                        <Text
                                            style={[
                                                styles.dayText,
                                                selectedDays.includes(i) && styles.dayTextActive,
                                            ]}
                                        >
                                            {day[0]}
                                        </Text>
                                    </TouchableOpacity>
                                ))}
                            </View>
                        </View>

                        <View style={styles.formGroup}>
                            <Text style={styles.label}>Début de la réservation entre</Text>
                            <Text style={styles.valueDisplay}>
                                {minutesToTime(timeRange[0])} — {minutesToTime(timeRange[1])}
                            </Text>
                            <Slider
                                value={timeRange}
                                onValueChange={setTimeRange}
                                minimumValue={480} // 08:00
                                maximumValue={1380} // 23:00
                                step={15}
                                minimumTrackTintColor="#1A1A1A"
                                thumbTintColor="#1A1A1A"
                            />
                        </View>

                        {isAlarm && (
                            <View style={styles.formGroup}>
                                <Text style={styles.label}>Nombre de semaines scanées:</Text>
                                <View style={styles.counterContainer}>
                                    <TouchableOpacity
                                        style={styles.counterButton}
                                        onPress={() => setPeriod(Math.max(1, period - 1))}
                                    >
                                        <Text style={styles.counterButtonText}>−</Text>
                                    </TouchableOpacity>
                                    <View style={styles.periodValueContainer}>
                                        <Text style={styles.periodValueText}>{period}</Text>
                                    </View>
                                    <TouchableOpacity
                                        style={styles.counterButton}
                                        onPress={() => setPeriod(Math.min(8, period + 1))}
                                    >
                                        <Text style={styles.counterButtonText}>+</Text>
                                    </TouchableOpacity>
                                </View>
                            </View>
                        )}

                        <View style={styles.formGroup}>
                            <Text style={styles.label}>Durées (h)</Text>
                            <View style={styles.row}>
                                {[
                                    { label: "1h", value: 3600 },
                                    { label: "1.5h", value: 5400 },
                                    { label: "2h", value: 7200 }
                                ].map((dur) => (
                                    <TouchableOpacity
                                        key={dur.value}
                                        style={[
                                            styles.chip,
                                            slotDurations.includes(dur.value) && styles.chipActive
                                        ]}
                                        onPress={() => toggleDuration(dur.value)}
                                    >
                                        <Text style={[
                                            styles.chipText,
                                            slotDurations.includes(dur.value) && styles.chipTextActive
                                        ]}>
                                            {dur.label}
                                        </Text>
                                    </TouchableOpacity>
                                ))}
                            </View>
                        </View>

                        <TouchableOpacity style={styles.submitButton} onPress={handleSubmit}>
                            <Text style={styles.submitButtonText}>
                                {isEditing ? "Enregistrer les modifications" : (isAlarm ? "Créer le créneau" : "Enregistrer le filtre")}

                            </Text>
                        </TouchableOpacity>

                        {isEditing && (
                            <TouchableOpacity
                                style={styles.deleteLink}
                                onPress={() => {
                                    onDelete(initialData.id);
                                    onClose();
                                }}
                            >
                                <Text style={styles.deleteLinkText}>Supprimer</Text>
                            </TouchableOpacity>
                        )}
                    </ScrollView>
                </View>
            </View>
        </Modal>
    );
}


const styles = StyleSheet.create({
    overlay: {
        flex: 1,
        justifyContent: "flex-end",
        backgroundColor: "rgba(0,0,0,0.4)",
    },
    backdrop: {
        ...StyleSheet.absoluteFillObject,
    },
    content: {
        backgroundColor: "#FFF",
        borderTopLeftRadius: 24,
        borderTopRightRadius: 24,
        maxHeight: "90%",
        paddingBottom: 40,
    },
    header: {
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
        padding: 24,
        borderBottomWidth: 1,
        borderBottomColor: "#F0F0F0",
    },
    title: {
        fontSize: 20,
        fontWeight: "700",
        color: "#1A1A1A",
    },
    closeIcon: {
        fontSize: 22,
        color: "#666",
    },
    scrollContent: {
        padding: 24,
    },
    formGroup: {
        marginBottom: 24,
    },
    label: {
        fontSize: 14,
        fontWeight: "600",
        color: "#666",
        marginBottom: 8,
        textTransform: "uppercase",
        letterSpacing: 0.5,
    },
    input: {
        borderBottomWidth: 1,
        borderBottomColor: "#E0E0E0",
        paddingVertical: 8,
        fontSize: 16,
        color: "#1A1A1A",
    },
    row: {
        flexDirection: "row",
        gap: 12,
    },
    chip: {
        paddingHorizontal: 16,
        paddingVertical: 8,
        borderRadius: 20,
        borderWidth: 1,
        borderColor: "#E0E0E0",
    },
    chipActive: {
        backgroundColor: "#1A1A1A",
        borderColor: "#1A1A1A",
    },
    chipText: {
        fontSize: 14,
        color: "#666",
    },
    chipTextActive: {
        color: "#FFF",
        fontWeight: "600",
    },
    weekdaysRow: {
        flexDirection: "row",
        justifyContent: "space-between",
    },
    dayCircle: {
        width: 36,
        height: 36,
        borderRadius: 18,
        borderWidth: 1,
        borderColor: "#E0E0E0",
        alignItems: "center",
        justifyContent: "center",
    },
    dayCircleActive: {
        backgroundColor: "#1A1A1A",
        borderColor: "#1A1A1A",
    },
    dayText: {
        fontSize: 14,
        color: "#666",
    },
    dayTextActive: {
        color: "#FFF",
        fontWeight: "600",
    },
    valueDisplay: {
        fontSize: 18,
        fontWeight: "700",
        color: "#1A1A1A",
        marginBottom: 8,
    },
    counterContainer: {
        flexDirection: 'row',
        alignItems: 'center',
        gap: 16,
        marginTop: 4,
    },
    counterButton: {
        width: 44,
        height: 44,
        borderRadius: 22,
        backgroundColor: '#F5F5F5',
        alignItems: 'center',
        justifyContent: 'center',
        borderWidth: 1,
        borderColor: '#E0E0E0',
    },
    counterButtonText: {
        fontSize: 20,
        fontWeight: '600',
        color: '#1A1A1A',
    },
    periodValueContainer: {
        minWidth: 40,
        alignItems: 'center',
    },
    periodValueText: {
        fontSize: 20,
        fontWeight: '700',
        color: '#1A1A1A',
    },
    submitButton: {
        backgroundColor: "#1A1A1A",
        paddingVertical: 16,
        borderRadius: 12,
        alignItems: "center",
        marginTop: 16,
    },
    submitButtonText: {
        color: "#FFF",
        fontSize: 16,
        fontWeight: "700",
    },
    deleteLink: {
        marginTop: 20,
        paddingVertical: 10,
        alignItems: 'center',
    },
    deleteLinkText: {
        color: '#FF4444',
        fontSize: 14,
        fontWeight: '600',
        textTransform: 'uppercase',
        letterSpacing: 1,
    },
});
