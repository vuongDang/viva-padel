import React, { useState, useEffect, useRef } from 'react';
import {
    StyleSheet,
    View,
    Text,
    Modal,
    TouchableOpacity,
    ScrollView,
    Clipboard,
    SafeAreaView
} from 'react-native';
import { Logger } from '../utils/logger';
import { theme } from '../styles/theme';

export default function DebugOverlay({ visible, onClose }) {
    const [logs, setLogs] = useState([]);
    const scrollViewRef = useRef();

    useEffect(() => {
        if (visible) {
            const unsubscribe = Logger.subscribe(newLogs => {
                setLogs(newLogs);
            });
            return unsubscribe;
        }
    }, [visible]);

    const handleCopy = () => {
        const text = logs.map(l => `[${l.timestamp}] [${l.type.toUpperCase()}] ${l.message}`).join('\n');
        Clipboard.setString(text);
        alert('Copié dans le presse-papiers');
    };

    const handleClear = () => {
        Logger.clear();
    };

    if (!visible) return null;

    return (
        <Modal
            visible={visible}
            animationType="slide"
            onRequestClose={onClose}
        >
            <SafeAreaView style={styles.container}>
                <View style={styles.header}>
                    <Text style={styles.title}>Console Debug</Text>
                    <TouchableOpacity onPress={onClose} style={styles.closeBtn}>
                        <Text style={styles.closeText}>×</Text>
                    </TouchableOpacity>
                </View>

                <ScrollView
                    ref={scrollViewRef}
                    style={styles.logList}
                    contentContainerStyle={styles.logListContent}
                    onContentSizeChange={() => scrollViewRef.current?.scrollToEnd({ animated: true })}
                >
                    {logs.length === 0 ? (
                        <Text style={styles.emptyText}>Aucun log pour le moment...</Text>
                    ) : (
                        logs.map((log) => (
                            <View key={log.id} style={styles.logItem}>
                                <Text style={styles.logTimestamp}>[{log.timestamp}]</Text>
                                <Text style={[styles.logMessage, styles[`${log.type}Text`]]}>
                                    {log.message}
                                </Text>
                            </View>
                        ))
                    )}
                </ScrollView>

                <View style={styles.footer}>
                    <TouchableOpacity style={[styles.button, styles.clearBtn]} onPress={handleClear}>
                        <Text style={styles.buttonText}>Effacer</Text>
                    </TouchableOpacity>
                    <TouchableOpacity style={[styles.button, styles.copyBtn]} onPress={handleCopy}>
                        <Text style={styles.buttonText}>Copier tout</Text>
                    </TouchableOpacity>
                </View>
            </SafeAreaView>
        </Modal>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#1A1A1A',
    },
    header: {
        flexDirection: 'row',
        justifyContent: 'space-between',
        alignItems: 'center',
        padding: 16,
        borderBottomWidth: 1,
        borderBottomColor: '#333',
    },
    title: {
        color: '#FFF',
        fontSize: 18,
        fontWeight: '700',
    },
    closeBtn: {
        padding: 8,
    },
    closeText: {
        color: '#FFF',
        fontSize: 24,
    },
    logList: {
        flex: 1,
    },
    logListContent: {
        padding: 12,
    },
    logItem: {
        marginBottom: 8,
        flexDirection: 'row',
    },
    logTimestamp: {
        color: '#888',
        fontSize: 11,
        marginRight: 6,
        fontFamily: 'monospace',
    },
    logMessage: {
        flex: 1,
        fontSize: 12,
        fontFamily: 'monospace',
    },
    logText: {
        color: '#DDD',
    },
    warnText: {
        color: '#FFCC00',
    },
    errorText: {
        color: '#FF4444',
    },
    emptyText: {
        color: '#666',
        textAlign: 'center',
        marginTop: 40,
    },
    footer: {
        flexDirection: 'row',
        padding: 16,
        gap: 12,
        backgroundColor: '#111',
    },
    button: {
        flex: 1,
        paddingVertical: 12,
        borderRadius: 8,
        alignItems: 'center',
    },
    buttonText: {
        color: '#FFF',
        fontWeight: '600',
    },
    clearBtn: {
        backgroundColor: '#444',
    },
    copyBtn: {
        backgroundColor: theme.colors.secondary,
    },
});
