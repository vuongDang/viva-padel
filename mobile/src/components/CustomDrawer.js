import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, Modal, Dimensions, Pressable } from 'react-native';
import LogoutConfirmationModal from './Modals/LogoutConfirmationModal';
import LoginModal from './Modals/LoginModal';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

const { width } = Dimensions.get('window');
const DRAWER_WIDTH = width * 0.7;

const CustomDrawer = React.memo(({ visible, onClose, onNavigate, currentScreen, onLogout, onLogin, user, onSimulateMatch, onShowDebug }) => {
    const insets = useSafeAreaInsets();
    const [logoutModalVisible, setLogoutModalVisible] = useState(false);
    const [loginModalVisible, setLoginModalVisible] = useState(false);

    const menuItems = [
        { id: 'Home', label: 'Accueil' },
        { id: 'Calendar', label: 'Calendrier' },
        { id: 'TimeSlots', label: 'Mes créneaux' },



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
                        <TouchableOpacity
                            activeOpacity={1}
                            onLongPress={() => {
                                onShowDebug();
                                onClose();
                            }}
                        >
                            <Text style={styles.headerTitle}>Viva Padel</Text>
                        </TouchableOpacity>
                        {user?.email && (
                            <Text style={styles.userEmail} numberOfLines={1} ellipsizeMode="tail">
                                {user.email}
                            </Text>
                        )}
                        <View style={styles.statusContainer}>
                            <View style={[styles.statusDot, user ? styles.statusDotConnected : styles.statusDotDisconnected]} />
                            <Text style={styles.statusText}>{user ? 'Connecté' : 'Non connecté'}</Text>
                        </View>
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

                    {user && (
                        <View style={styles.logoutSection}>
                            {__DEV__ && (
                                <TouchableOpacity
                                    style={styles.testItem}
                                    onPress={() => {
                                        const mockData = {
                                            "toto": {
                                                "2026-01-22": {
                                                    "courts": [
                                                        {
                                                            "name": "Court 1",
                                                            "indoor": true,
                                                            "slots": [
                                                                { "start_at": "18:00", "prices": [{ "bookable": true }] },
                                                                { "start_at": "19:30", "prices": [{ "bookable": true }] }
                                                            ]
                                                        }
                                                    ]
                                                },
                                                "2026-01-27": {
                                                    "courts": [
                                                        {
                                                            "name": "Court 1",
                                                            "indoor": true,
                                                            "slots": [
                                                                { "start_at": "18:00", "prices": [{ "bookable": true }] },
                                                                { "start_at": "19:30", "prices": [{ "bookable": true }] }
                                                            ]
                                                        }
                                                    ]
                                                }

                                            }
                                        };
                                        onSimulateMatch(mockData);
                                        onNavigate('TimeSlots');
                                        onClose();

                                    }}
                                >
                                    <Text style={[styles.testLabel, { color: '#f2994a' }]}>Simuler Match (Debug)</Text>
                                </TouchableOpacity>
                            )}

                            <TouchableOpacity
                                style={styles.logoutItem}
                                onPress={() => setLogoutModalVisible(true)}
                            >
                                <Text style={styles.logoutLabel}>Déconnexion</Text>
                            </TouchableOpacity>

                            <LogoutConfirmationModal
                                visible={logoutModalVisible}
                                onClose={() => setLogoutModalVisible(false)}
                                onConfirm={() => {
                                    setLogoutModalVisible(false);
                                    onLogout();
                                    onClose();
                                }}
                            />
                        </View>
                    )}

                    {!user && (
                        <View style={styles.logoutSection}>
                            <TouchableOpacity
                                style={styles.logoutItem}
                                onPress={() => setLoginModalVisible(true)}
                            >
                                <Text style={[styles.logoutLabel, { color: '#1A1A1A' }]}>Connexion</Text>
                            </TouchableOpacity>

                            <LoginModal
                                visible={loginModalVisible}
                                onClose={() => setLoginModalVisible(false)}
                                onLogin={onLogin}
                            />
                        </View>
                    )}

                    <View style={[styles.footer, { paddingBottom: insets.bottom + 16 }]}>
                        <Text style={styles.footerText}>v1.0.0</Text>
                    </View>
                </View>
            </View>
        </Modal>
    );
});

export default CustomDrawer;

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
    userEmail: {
        fontSize: 13,
        color: '#666',
        marginTop: 2,
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
    logoutSection: {
        borderTopWidth: 1,
        borderTopColor: '#E8E8E8',
        paddingVertical: 8,
    },
    logoutItem: {
        paddingVertical: 14,
        paddingHorizontal: 24,
    },
    logoutLabel: {
        fontSize: 15,
        color: '#FF4444',
        fontWeight: '600',
    },
    statusContainer: {
        flexDirection: 'row',
        alignItems: 'center',
        marginTop: 4,
    },
    statusDot: {
        width: 8,
        height: 8,
        borderRadius: 4,
        marginRight: 6,
    },
    statusDotConnected: {
        backgroundColor: '#34A853',
    },
    statusDotDisconnected: {
        backgroundColor: '#999',
    },
    statusText: {
        fontSize: 12,
        color: '#666',
        fontWeight: '500',
    },
});

