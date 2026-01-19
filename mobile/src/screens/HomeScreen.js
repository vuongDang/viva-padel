import React from 'react';
import { StyleSheet, View, Text, ScrollView, TouchableOpacity } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { theme } from '../styles/theme';

export default function HomeScreen({ navigation, openDrawer }) {
  const currentDate = new Date().toLocaleDateString('fr-FR', {
    weekday: 'long',
    day: 'numeric',
    month: 'long'
  });

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <TouchableOpacity style={styles.menuButton} onPress={openDrawer}>
          <Text style={styles.menuIcon}>☰</Text>
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Viva Padel</Text>
      </View>

      <ScrollView contentContainerStyle={styles.content}>
        <View style={styles.welcomeSection}>
          <Text style={styles.dateText}>{currentDate}</Text>
          <Text style={styles.welcomeTitle}>Bonjour</Text>
        </View>

        <Text style={styles.sectionTitle}>Actions</Text>

        <TouchableOpacity
          style={styles.actionCard}
          onPress={() => navigation.navigate('Reservations')}
        >
          <Text style={styles.actionTitle}>Réserver un terrain</Text>
          <Text style={styles.actionDesc}>Voir les disponibilités</Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={styles.actionCard}
          onPress={() => navigation.navigate('Alarms')}
        >
          <Text style={styles.actionTitle}>Gérer les alarmes</Text>
          <Text style={styles.actionDesc}>Configurer les alertes</Text>
        </TouchableOpacity>
      </ScrollView>
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
  content: {
    padding: 20,
  },
  welcomeSection: {
    marginBottom: 24,
  },
  dateText: {
    fontSize: 13,
    color: '#888',
    textTransform: 'capitalize',
    marginBottom: 4,
  },
  welcomeTitle: {
    fontSize: 28,
    fontWeight: '700',
    color: '#1A1A1A',
  },
  sectionTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#333',
    marginBottom: 12,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  actionCard: {
    backgroundColor: '#FFF',
    padding: 16,
    borderRadius: 12,
    marginBottom: 12,
    borderWidth: 1,
    borderColor: '#E8E8E8',
  },
  actionTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1A1A1A',
  },
  actionDesc: {
    fontSize: 13,
    color: '#888',
    marginTop: 4,
  },
});
