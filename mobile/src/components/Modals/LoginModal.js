import React, { useState } from "react";
import { AuthService } from "../../services/authService";
import {
    StyleSheet,
    View,
    Text,
    Modal,
    TouchableOpacity,
    TextInput,
    KeyboardAvoidingView,
    Platform,
} from "react-native";

export default function LoginModal({ visible, onClose, onLogin }) {
    const [email, setEmail] = useState("");
    const [isSignup, setIsSignup] = useState(false);

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState("");

    const handleSubmit = async () => {
        if (!email.trim()) return;
        setLoading(true);
        setError("");

        try {
            if (isSignup) {
                await AuthService.signup(email);
                // Switch to login mode after successful signup
                setIsSignup(false);
                alert("Compte créé ! Vous pouvez maintenant vous connecter.");
            } else {
                const response = await AuthService.login(email);
                onLogin(email, response.token);
                onClose();
            }
        } catch (err) {
            setError(err.message || "Une erreur est survenue");
        } finally {
            setLoading(false);
        }
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
                <KeyboardAvoidingView
                    behavior={Platform.OS === "ios" ? "padding" : "height"}
                    style={styles.content}
                >
                    <View style={styles.header}>
                        <Text style={styles.title}>{isSignup ? "Inscription" : "Connexion"}</Text>
                        <TouchableOpacity onPress={onClose}>
                            <Text style={styles.closeIcon}>✕</Text>
                        </TouchableOpacity>
                    </View>

                    <View style={styles.form}>
                        <Text style={styles.label}>Email</Text>
                        <TextInput
                            style={styles.input}
                            placeholder="votre@email.com"
                            value={email}
                            onChangeText={setEmail}
                            keyboardType="email-address"
                            autoCapitalize="none"
                            placeholderTextColor="#AAA"
                        />

                        {error ? <Text style={styles.errorText}>{error}</Text> : null}

                        <TouchableOpacity
                            style={[styles.submitButton, loading && styles.disabledButton]}
                            onPress={handleSubmit}
                            disabled={loading}
                        >
                            <Text style={styles.submitButtonText}>
                                {loading ? "Chargement..." : (isSignup ? "Créer un compte" : "Se connecter")}
                            </Text>
                        </TouchableOpacity>

                        <TouchableOpacity
                            style={styles.switchButton}
                            onPress={() => setIsSignup(!isSignup)}
                        >
                            <Text style={styles.switchButtonText}>
                                {isSignup ? "Déjà un compte ? Se connecter" : "Pas de compte ? S'inscrire"}
                            </Text>
                        </TouchableOpacity>
                    </View>
                </KeyboardAvoidingView>
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
    form: {
        padding: 24,
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
        paddingVertical: 12,
        fontSize: 16,
        color: "#1A1A1A",
        marginBottom: 32,
    },
    submitButton: {
        backgroundColor: "#1A1A1A",
        paddingVertical: 16,
        borderRadius: 12,
        alignItems: "center",
        marginBottom: 16,
    },
    submitButtonText: {
        color: "#FFF",
        fontSize: 16,
        fontWeight: "700",
    },
    switchButton: {
        alignItems: "center",
        paddingVertical: 8,
    },
    switchButtonText: {
        color: "#666",
        fontSize: 14,
        fontWeight: "500",
    },
    errorText: {
        color: "#FF3B30",
        fontSize: 14,
        marginBottom: 16,
        textAlign: "center",
    },
    disabledButton: {
        backgroundColor: "#666",
    },
});
