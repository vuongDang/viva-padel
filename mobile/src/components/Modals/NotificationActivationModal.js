import React, { useState, useEffect } from "react";
import {
    StyleSheet,
    View,
    Text,
    Modal,
    TouchableOpacity,
    ScrollView,
    ActivityIndicator,
    Alert
} from "react-native";

export default function NotificationActivationModal({ visible, onClose, alarms, onToggleAlarm, onSync }) {
    const [syncing, setSyncing] = useState(false);
    const [weeksAhead, setWeeksAhead] = useState(1);

    const handleSync = async () => {
        setSyncing(true);
        try {
            await onSync(alarms, weeksAhead);
            Alert.alert("Succès", "Vos notifications ont été synchronisées avec le serveur.");
            onClose();
        } catch (error) {
            Alert.alert("Erreur", error.message || "Impossible de synchroniser les notifications.");
        } finally {
            setSyncing(false);
        }
    };

    const handleDeactivateAll = async () => {
        setSyncing(true);
        try {
            await onSync([], weeksAhead);
            Alert.alert("Succès", "Toutes les notifications ont été désactivées sur le serveur.");
            onClose();
        } catch (error) {
            Alert.alert("Erreur", error.message || "Impossible de désactiver les notifications.");
        } finally {
            setSyncing(false);
        }
    };


    const incrementWeeks = () => setWeeksAhead(prev => Math.min(prev + 1, 4));
    const decrementWeeks = () => setWeeksAhead(prev => Math.max(prev - 1, 1));

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
                        <Text style={styles.title}>Activer les notifications</Text>
                        <TouchableOpacity onPress={onClose}>
                            <Text style={styles.closeIcon}>✕</Text>
                        </TouchableOpacity>
                    </View>

                    <ScrollView showsVerticalScrollIndicator={false} contentContainerStyle={styles.scrollContent}>
                        <Text style={styles.description}>
                            Recevez des notifications pour des terrains qui se libèrent sur vos créneaux favoris.
                        </Text>

                        {/* Weeks Ahead Counter */}
                        <View style={styles.settingsSection}>
                            <View style={styles.counterContainer}>
                                <TouchableOpacity style={styles.counterBtn} onPress={decrementWeeks}>
                                    <Text style={styles.counterBtnText}>−</Text>
                                </TouchableOpacity>
                                <Text style={styles.counterValue}>{weeksAhead}</Text>
                                <TouchableOpacity style={styles.counterBtn} onPress={incrementWeeks}>
                                    <Text style={styles.counterBtnText}>+</Text>
                                </TouchableOpacity>
                            </View>

                            <Text style={styles.settingsLabel}>prochaine(s) semaine(s)</Text>
                        </View>

                        <View style={styles.divider} />

                        <Text style={styles.settingsLabel}>Notifications pour:</Text>
                        {alarms.map((alarm) => (

                            <TouchableOpacity
                                key={alarm.id}
                                style={styles.alarmRow}
                                onPress={() => onToggleAlarm(alarm.id)}
                            >
                                <View style={[styles.checkbox, alarm.activated && styles.checkboxChecked]}>
                                    {alarm.activated && <Text style={styles.checkboxIcon}>✓</Text>}
                                </View>
                                <Text style={[styles.alarmName, alarm.activated && styles.alarmNameActive]}>
                                    {alarm.name}
                                </Text>
                            </TouchableOpacity>
                        ))}

                        <TouchableOpacity
                            style={[styles.syncButton, syncing && styles.disabledButton]}
                            onPress={handleSync}
                            disabled={syncing}
                        >
                            {syncing ? (
                                <ActivityIndicator color="#FFF" />
                            ) : (
                                <Text style={styles.syncButtonText}>Synchroniser avec le serveur</Text>
                            )}
                        </TouchableOpacity>

                        <TouchableOpacity
                            style={[styles.deactivateButton, syncing && styles.disabledButton]}
                            onPress={handleDeactivateAll}
                            disabled={syncing}
                        >
                            <Text style={styles.deactivateButtonText}>Désactiver les notifications</Text>
                        </TouchableOpacity>

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
        maxHeight: "80%",
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
    description: {
        fontSize: 14,
        color: "#666",
        lineHeight: 20,
        marginBottom: 24,
    },
    settingsSection: {
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
        marginBottom: 20,
    },
    settingsLabel: {
        fontSize: 15,
        fontWeight: "600",
        color: "#1A1A1A",
    },
    counterContainer: {
        flexDirection: "row",
        alignItems: "center",
        gap: 16,
    },
    counterBtn: {
        width: 32,
        height: 32,
        borderRadius: 16,
        borderWidth: 1,
        borderColor: "#E0E0E0",
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: "#F9F9F9",
    },
    counterBtnText: {
        fontSize: 20,
        color: "#1A1A1A",
        lineHeight: 22,
    },
    counterValue: {
        fontSize: 16,
        fontWeight: "700",
        color: "#1A1A1A",
        minWidth: 20,
        textAlign: "center",
    },
    divider: {
        height: 1,
        backgroundColor: "#F0F0F0",
        marginBottom: 20,
    },
    alarmRow: {

        flexDirection: "row",
        alignItems: "center",
        paddingVertical: 12,
        borderBottomWidth: 1,
        borderBottomColor: "#F5F5F5",
    },
    checkbox: {
        width: 24,
        height: 24,
        borderRadius: 6,
        borderWidth: 2,
        borderColor: "#E0E0E0",
        marginRight: 16,
        alignItems: "center",
        justifyContent: "center",
    },
    checkboxChecked: {
        backgroundColor: "#1A73E8",
        borderColor: "#1A73E8",
    },
    checkboxIcon: {
        color: "#FFF",
        fontSize: 14,
        fontWeight: "bold",
    },
    alarmName: {
        fontSize: 16,
        color: "#444",
        fontWeight: "400",
    },
    alarmNameActive: {
        color: "#1A1A1A",
        fontWeight: "500",
    },
    syncButton: {
        backgroundColor: "#1A1A1A",
        paddingVertical: 16,
        borderRadius: 12,
        alignItems: "center",
        marginTop: 32,
    },
    syncButtonText: {
        color: "#FFF",
        fontSize: 16,
        fontWeight: "700",
    },
    deactivateButton: {
        paddingVertical: 12,
        alignItems: "center",
        marginTop: 12,
    },
    deactivateButtonText: {
        color: "#FF4444",
        fontSize: 14,
        fontWeight: "600",
        textTransform: "uppercase",
        letterSpacing: 0.5,
    },
    disabledButton: {
        opacity: 0.6,
    },

});
