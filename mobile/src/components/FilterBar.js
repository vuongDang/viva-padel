import React from "react";
import {
  StyleSheet,
  View,
  Text,
  ScrollView,
  TouchableOpacity,
} from "react-native";
import { theme } from "../styles/theme";

export default function FilterBar({
  filters,
  activeFilterId,
  onSelectFilter,
  onDeleteMode,
  isDeleteMode,
  onEditMode,
  isEditMode,
  onCreateFilter,
}) {

  return (
    <View style={styles.container}>
      <View style={styles.fixedHeader}>
        <Text style={styles.label}>Créneaux</Text>

        <TouchableOpacity style={styles.actionBtn} onPress={onCreateFilter}>
          <Text style={styles.actionBtnText}>+</Text>
        </TouchableOpacity>

        {/* Edit Toggle */}
        <TouchableOpacity
          style={[styles.actionBtn, isEditMode && styles.actionBtnActive]}
          onPress={onEditMode}
        >
          <Text style={[styles.actionBtnText, isEditMode && styles.actionBtnTextActive, { fontSize: 16 }]}>✎</Text>
        </TouchableOpacity>

        {/* Delete Toggle */}
        <TouchableOpacity
          style={[styles.actionBtn, isDeleteMode && styles.actionBtnActive]}
          onPress={onDeleteMode}
        >
          <Text style={[styles.actionBtnText, isDeleteMode && styles.actionBtnTextActive, { fontSize: 16, marginTop: -1 }]}>✕</Text>
        </TouchableOpacity>
      </View>

      <View style={styles.tagsContainer}>
        <TouchableOpacity
          style={[
            styles.filterBtn,
            activeFilterId === "all" && styles.filterBtnActive,
          ]}
          onPress={() => onSelectFilter("all")}
        >
          <Text
            style={[
              styles.filterText,
              activeFilterId === "all" && styles.filterTextActive,
            ]}
          >
            Tous

          </Text>
        </TouchableOpacity>

        {filters.map((filter) => (
          <TouchableOpacity
            key={filter.id}
            style={[
              styles.filterBtn,
              activeFilterId === filter.id && styles.filterBtnActive,
              isDeleteMode && styles.filterBtnDelete,
              isEditMode && styles.filterBtnEdit,
            ]}
            onPress={() => onSelectFilter(filter.id)}
          >
            <Text
              style={[
                styles.filterText,
                activeFilterId === filter.id && styles.filterTextActive,
                isDeleteMode && styles.filterTextDelete,
                isEditMode && styles.filterTextEdit,
              ]}
            >
              {isDeleteMode ? "× " : isEditMode ? "✎ " : ""}
              {filter.name || "Sans nom"}

            </Text>
          </TouchableOpacity>
        ))}
      </View>
    </View>

  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: "column",
    paddingVertical: 12,
    paddingHorizontal: 16,
    backgroundColor: theme.colors.headerBg,
    marginHorizontal: 16,
    marginVertical: 12,
    borderRadius: 20,
    gap: 12,
  },

  fixedHeader: {
    flexDirection: "row",
    alignItems: "center",
    width: '100%',
  },

  label: {
    fontWeight: "600",
    fontSize: 14,
    color: theme.colors.grayText,
    marginRight: 8,
  },
  actionBtn: {
    width: 28,
    height: 28,
    borderRadius: 14,
    borderWidth: 1,
    borderColor: theme.colors.border,
    backgroundColor: theme.colors.background,
    alignItems: "center",
    justifyContent: "center",
    marginRight: 8,
  },
  actionBtnActive: {
    backgroundColor: '#333',
    borderColor: '#333',
  },



  actionBtnText: {
    color: theme.colors.grayText,
    fontSize: 18,
    marginTop: -2,
  },
  actionBtnTextActive: {
    color: "white",
  },
  tagsContainer: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 8,
    width: '100%',
  },


  filterBtn: {
    paddingVertical: 6,
    paddingHorizontal: 12,
    borderRadius: 14,
    borderWidth: 1,
    borderColor: "#DDD",
    backgroundColor: "#FFF",
  },


  filterBtnActive: {
    backgroundColor: theme.colors.availableLight,
    borderColor: "#ceead6",
  },
  filterBtnDelete: {
    backgroundColor: "#F0F0F0",
    borderColor: "#E0E0E0",
  },

  filterText: {
    fontSize: 14,
    fontWeight: "600",
    color: "#000000",
  },


  filterTextActive: {
    color: "#137333",
    fontWeight: "600",
  },
  filterTextDelete: {
    color: "#333",
    fontWeight: "600",
  },
  filterBtnEdit: {
    backgroundColor: "#F0F0F0",
    borderColor: "#E0E0E0",
  },
  filterTextEdit: {
    color: "#333",
    fontWeight: "600",
  },



});
