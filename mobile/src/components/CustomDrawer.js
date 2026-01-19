import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity, Modal, Dimensions, Pressable } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

const { width } = Dimensions.get('window');
const DRAWER_WIDTH = width * 0.7;

export default function CustomDrawer({ visible, onClose, onNavigate, currentScreen }) {
    const insets = useSafeAreaInsets();

    const menuItems = [
        { id: 'Home', label: 'Accueil' },
        { id: 'Reservations', label: 'Réservations' },
        { id: 'Alarms', label: 'Alarmes' },
    ];

    const handleNavigate = (screenId) => {
        onNavigate(screenId);
        onClose();
    };

    if (!visible) return null;

    return (
        <Modal
            visible={visible}
            transparent={true}
            animationType="none"
            onRequestClose={onClose}
            statusBarTranslucent
        >
            <View style={styles.overlay}>
                <Pressable style={styles.backdrop} onPress={onClose} />

                <View style={[styles.drawer, { paddingTop: insets.top }]}>
                    <View style={styles.header}>
                        <Text style={styles.headerTitle}>Viva Padel</Text>
                    </View>

                    <View style={styles.menuList}>
                        {menuItems.map((item) => (
                            <TouchableOpacity
                                key={item.id}
                                style={[
                                    styles.menuItem,
                                    currentScreen === item.id && styles.menuItemActive
                                ]}
                                onPress={() => handleNavigate(item.id)}
                                activeOpacity={0.6}
                            >
                                <Text style={[
                                    styles.menuLabel,
                                    currentScreen === item.id && styles.menuLabelActive
                                ]}>
                                    {item.label}
                                </Text>
                            </TouchableOpacity>
                        ))}
                    </View>

                    <View style={[styles.footer, { paddingBottom: insets.bottom + 16 }]}>
                        <Text style={styles.footerText}>v1.0.0</Text>
                    </View>
                </View>
            </View>
        </Modal>
    );
}

const styles = StyleSheet.create({
    overlay: {
        flex: 1,
        flexDirection: 'row',
        backgroundColor: 'rgba(0,0,0,0.4)',
    },
    backdrop: {
        flex: 1,
    },
    drawer: {
        position: 'absolute',
        left: 0,
        top: 0,
        bottom: 0,
        width: DRAWER_WIDTH,
        backgroundColor: '#FFF',
    },
    header: {
        padding: 24,
        borderBottomWidth: 1,
        borderBottomColor: '#E8E8E8',
    },
    headerTitle: {
        fontSize: 20,
        fontWeight: '700',
        color: '#1A1A1A',
    },
    menuList: {
        flex: 1,
        paddingTop: 8,
    },
    menuItem: {
        paddingVertical: 14,
        paddingHorizontal: 24,
    },
    menuItemActive: {
        backgroundColor: '#F5F5F5',
    },
    menuLabel: {
        fontSize: 15,
        color: '#333',
        fontWeight: '500',
    },
    menuLabelActive: {
        color: '#1A1A1A',
        fontWeight: '600',
    },
    footer: {
        padding: 24,
        borderTopWidth: 1,
        borderTopColor: '#E8E8E8',
    },
    footerText: {
        fontSize: 12,
        color: '#AAA',
    },
});
