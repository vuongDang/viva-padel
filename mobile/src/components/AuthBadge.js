import React, { useState } from 'react';
import { StyleSheet, View, Text, TouchableOpacity } from 'react-native';
import LoginModal from './Modals/LoginModal';

export default function AuthBadge({ user, onLogin, onLogout }) {
    const [loginModalVisible, setLoginModalVisible] = useState(false);

    const handleAuthPress = () => {
        if (user) {
            onLogout();
        } else {
            setLoginModalVisible(true);
        }
    };

    return (
        <>
            {user ? (
                <TouchableOpacity style={styles.connectedBadge} onPress={handleAuthPress}>
                    <View style={styles.statusDot} />
                    <Text style={styles.connectedText}>Connecté</Text>
                </TouchableOpacity>
            ) : (
                <TouchableOpacity
                    style={styles.loginButton}
                    onPress={handleAuthPress}
                >
                    <Text style={styles.loginButtonText}>Connexion</Text>
                </TouchableOpacity>
            )}

            <LoginModal
                visible={loginModalVisible}
                onClose={() => setLoginModalVisible(false)}
                onLogin={onLogin}
            />
        </>
    );
}

const styles = StyleSheet.create({
    loginButton: {
        paddingHorizontal: 12,
        paddingVertical: 6,
        borderRadius: 6,
        backgroundColor: '#F0F0F0',
    },
    loginButtonText: {
        fontSize: 13,
        fontWeight: '600',
        color: '#1A1A1A',
    },
    connectedBadge: {
        flexDirection: 'row',
        alignItems: 'center',
        backgroundColor: '#E6F4EA',
        paddingHorizontal: 10,
        paddingVertical: 4,
        borderRadius: 12,
    },
    statusDot: {
        width: 6,
        height: 6,
        borderRadius: 3,
        backgroundColor: '#34A853',
        marginRight: 6,
    },
    connectedText: {
        fontSize: 12,
        fontWeight: '600',
        color: '#137333',
    },
});
