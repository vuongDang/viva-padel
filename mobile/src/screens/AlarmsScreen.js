import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, ScrollView, Switch } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import CreationModal from '../components/Modals/CreationModal';

const WEEKDAYS_SHORT = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

export default function AlarmsScreen({ navigation, openDrawer }) {
    const [alarms, setAlarms] = useState([]);
    const [createModalVisible, setCreateModalVisible] = useState(false);
    const [editingAlarm, setEditingAlarm] = useState(null);

    const formatDays = (days) => {
        if (!days || days.length === 0) return "";
        if (days.length === 7) return "Tous les jours";
        return days.map(d => WEEKDAYS_SHORT[d]).join(', ');
    };

    const handleSaveAlarm = (alarmConfig) => {
        let finalName = alarmConfig.name;
        const otherAlarms = alarms.filter(a => a.id !== alarmConfig.id);

        // check if name already exists
        let counter = 1;
        while (otherAlarms.some(a => a.name === finalName)) {
            finalName = `${alarmConfig.name} ${counter}`;
            counter++;
        }

        const configWithUniqueName = { ...alarmConfig, name: finalName };

        if (alarmConfig.id) {
            // Update existing alarm
            setAlarms(alarms.map(a => a.id === alarmConfig.id ? configWithUniqueName : a));
        } else {
            // Create new alarm
            const alarmWithId = {
                ...configWithUniqueName,
                id: Date.now().toString(),
            };
            setAlarms([...alarms, alarmWithId]);
        }
    };

    const openEditModal = (alarm) => {
        setEditingAlarm(alarm);
        setCreateModalVisible(true);
    };

    const openCreateModal = () => {
        setEditingAlarm(null);
        setCreateModalVisible(true);
    };

    const deleteAlarm = (id) => {
        setAlarms(alarms.filter(alarm => alarm.id !== id));
    };

    const toggleAlarm = (id) => {
        setAlarms(alarms.map(alarm =>
            alarm.id === id ? { ...alarm, activated: !alarm.activated } : alarm
        ));
    };

    return (
        <SafeAreaView style={styles.container}>
            <View style={styles.header}>
                <TouchableOpacity style={styles.menuButton} onPress={openDrawer}>
                    <Text style={styles.menuIcon}>☰</Text>
                </TouchableOpacity>
                <Text style={styles.headerTitle}>Alarmes</Text>
            </View>

            <ScrollView contentContainerStyle={styles.scrollContent}>
                {alarms.length === 0 ? (
                    <View style={styles.emptyContent}>
                        <Text style={styles.title}>Aucune alarme</Text>
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
                            </View>
                        ))}
                    </View>
                )}

                <View style={styles.buttonContainer}>
                    <TouchableOpacity
                        style={[styles.button, styles.primaryButton]}
                        onPress={openCreateModal}
                    >
                        <Text style={styles.primaryButtonText}>Créer</Text>
                    </TouchableOpacity>

                    <TouchableOpacity style={[styles.button, styles.secondaryButton]}>
                        <Text style={styles.secondaryButtonText}>Activer</Text>
                    </TouchableOpacity>
                </View>
            </ScrollView>

            <CreationModal
                visible={createModalVisible}
                onClose={() => {
                    setCreateModalVisible(false);
                    setEditingAlarm(null);
                }}
                onCreate={handleSaveAlarm}
                onDelete={deleteAlarm}
                mode="alarm"
                initialData={editingAlarm}
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
        alignItems: 'center',
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
});
