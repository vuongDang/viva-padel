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
  onCreateFilter,
}) {
  return (
    <View style={styles.container}>
      <View style={styles.tagsContainer}>
        <View style={styles.fixedHeader}>
          <Text style={styles.label}>Filtres</Text>

          <TouchableOpacity style={styles.actionBtn} onPress={onCreateFilter}>
            <Text style={styles.actionBtnText}>+</Text>
          </TouchableOpacity>

          <TouchableOpacity
            style={[styles.actionBtn, isDeleteMode && styles.actionBtnActive]}
            onPress={onDeleteMode}
          >
            <Text
              style={[
                styles.actionBtnText,
                isDeleteMode && styles.actionBtnTextActive,
              ]}
            >
              -
            </Text>
          </TouchableOpacity>
        </View>

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
            ]}
            onPress={() => onSelectFilter(filter.id)}
          >
            <Text
              style={[
                styles.filterText,
                activeFilterId === filter.id && styles.filterTextActive,
                isDeleteMode && styles.filterTextDelete,
              ]}
            >
              {isDeleteMode ? "× " : ""}
              {filter.name}
            </Text>
          </TouchableOpacity>
        ))}
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: "row",
    flexWrap: "wrap",
    alignItems: "center",
    paddingVertical: 12,
    paddingHorizontal: 16,
    backgroundColor: theme.colors.headerBg,
    marginHorizontal: 16,
    marginVertical: 12,
    borderRadius: 30,
    gap: 8,
  },
  fixedHeader: {
    flexDirection: "row",
    alignItems: "center",
    marginRight: 4,
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
    backgroundColor: theme.colors.unavailable,
    borderColor: theme.colors.unavailable,
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
    flex: 1,
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 8,
  },
  filterBtn: {
    paddingVertical: 6,
    paddingHorizontal: 8,
    borderRadius: 16,
    borderWidth: 1,
    borderColor: theme.colors.border,
    backgroundColor: theme.colors.background,
    marginLeft: 0,
  },
  filterBtnActive: {
    backgroundColor: theme.colors.availableLight,
    borderColor: "#ceead6",
  },
  filterBtnDelete: {
    backgroundColor: theme.colors.unavailableLight,
    borderColor: "#fad2cf",
  },
  filterText: {
    fontSize: 13,
    fontWeight: "500",
    color: theme.colors.text,
  },
  filterTextActive: {
    color: "#137333",
    fontWeight: "600",
  },
  filterTextDelete: {
    color: "#d93025",
  },
});
