import React from "react";
import {
  StyleSheet,
  View,
  Text,
  Modal,
  TouchableOpacity,
  ScrollView,
} from "react-native";
import { theme } from "../../styles/theme";

export default function BookingModal({ visible, slotGroup, onClose }) {
  if (!visible || !slotGroup) return null;

  const sortedDurations = Array.from(slotGroup.durations).sort();
  const durationText = sortedDurations
    .map((d) => {
      const mins = d / 60;
      if (mins === 60) return "1h";
      if (mins === 90) return "1h30";
      if (mins === 120) return "2h";
      if (mins === 180) return "3h";
    })
    .join(" ou ");

  return (
    <Modal
      animationType="fade"
      transparent={true}
      visible={visible}
      onRequestClose={onClose}
    >
      <TouchableOpacity
        style={styles.overlay}
        activeOpacity={1}
        onPress={onClose}
      >
        <View style={styles.content} onStartShouldSetResponder={() => true}>
          <TouchableOpacity style={styles.closeBtn} onPress={onClose}>
            <Text style={styles.closeText}>&times;</Text>
          </TouchableOpacity>
          <ScrollView
            style={{ width: "100%" }}
            contentContainerStyle={{ alignItems: "center" }}
          >
            <Text style={styles.title}>Détails du créneau</Text>

            <View style={styles.details}>
              <Text style={styles.detailText}>
                <Text style={styles.bold}>Terrain :</Text> {slotGroup.courtName}
              </Text>
              <Text style={styles.detailText}>
                <Text style={styles.bold}>Heure :</Text> {slotGroup.startAt}
              </Text>
              <Text style={styles.detailText}>
                <Text style={styles.bold}>Durées disponibles :</Text>{" "}
                {durationText}
              </Text>
            </View>

            <TouchableOpacity style={styles.reserveBtn} disabled={true}>
              <Text style={styles.reserveBtnText}>Réserver</Text>
            </TouchableOpacity>
          </ScrollView>
        </View>
      </TouchableOpacity>
    </Modal>
  );
}

const styles = StyleSheet.create({
  overlay: {
    flex: 1,
    backgroundColor: "rgba(0,0,0,0.5)",
    justifyContent: "center",
    alignItems: "center",
  },
  content: {
    backgroundColor: "white",
    width: "85%",
    maxWidth: 400,
    borderRadius: 12,
    padding: 24,
    alignItems: "center",
  },
  closeBtn: {
    position: "absolute",
    paddingHorizontal: 14,
    top: 0,
    right: 0,
  },
  closeText: {
    fontSize: 40,
    color: "#70757a",
  },
  title: {
    fontSize: 20,
    fontWeight: "600",
    marginBottom: 20,
    textAlign: "center",
  },
  details: {
    marginBottom: 24,
    width: "100%",
  },
  detailText: {
    fontSize: 16,
    marginBottom: 8,
    color: theme.colors.text,
  },
  bold: {
    fontWeight: "700",
  },
  reserveBtn: {
    width: "100%",
    padding: 14,
    borderRadius: 8,
    backgroundColor: "#e8eaed", // Disabled by default as per frontend logic
    alignItems: "center",
  },
  reserveBtnText: {
    fontSize: 16,
    fontWeight: "600",
    color: "#9aa0a6",
  },
});
