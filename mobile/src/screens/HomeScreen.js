import React, { useState } from 'react';
import { StyleSheet, View, Text, ScrollView, TouchableOpacity } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { theme } from '../styles/theme';
import AuthBadge from '../components/AuthBadge';


export default function HomeScreen({ navigation, openDrawer, user, onLogout, onLogin }) {
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
        <View style={styles.headerSpacer} />
        <AuthBadge user={user} onLogin={onLogin} onLogout={onLogout} />


      </View>

      <ScrollView contentContainerStyle={styles.content}>
        <View style={styles.welcomeSection}>
          <Text style={styles.dateText}>{currentDate}</Text>
          <Text style={styles.welcomeTitle}>{user ? `Bonjour ${user.email.split('@')[0]}` : 'Bonjour'}</Text>
        </View>

        <Text style={styles.sectionTitle}>Actions</Text>

        <TouchableOpacity
          style={styles.actionCard}
          onPress={() => navigation.navigate('Calendar')}

        >
          <Text style={styles.actionTitle}>Réserver un terrain</Text>
          <Text style={styles.actionDesc}>Voir le calendrier</Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={styles.actionCard}
          onPress={() => navigation.navigate('TimeSlots')}

        >
          <Text style={styles.actionTitle}>Mes créneaux</Text>
          <Text style={styles.actionDesc}>Configurer les alertes</Text>
        </TouchableOpacity>

      </ScrollView>

      {/* AuthBadge handles its own login modal internally */}
    </SafeAreaView>

  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: theme.colors.background,
  },
  header: theme.styles.header,

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
  headerSpacer: {
    flex: 1,
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
    color: theme.colors.text,
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
    backgroundColor: theme.colors.card,
    padding: 20,
    borderRadius: 16,
    marginBottom: 16,
    ...theme.shadows.small,
  },
  actionTitle: {
    fontSize: 18,
    fontWeight: '700',
    color: theme.colors.text,
  },
  actionDesc: {
    fontSize: 14,
    color: theme.colors.textSecondary,
    marginTop: 4,
  },


});

