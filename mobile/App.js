import React, { useEffect, useState, useCallback, useRef } from 'react';
import { Modal, View, Text, StyleSheet, TouchableOpacity, Alert } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import HomeScreen from './src/screens/HomeScreen';
import ReservationsScreen from './src/screens/ReservationsScreen';
import AlarmsScreen from './src/screens/AlarmsScreen';
import CustomDrawer from './src/components/CustomDrawer';
import * as Notifications from 'expo-notifications';
import { NotificationService } from './src/services/notificationService';

const Stack = createNativeStackNavigator();

export default function App() {
  const [drawerVisible, setDrawerVisible] = useState(false);
  const [currentScreen, setCurrentScreen] = useState('Home');
  const [modalVisible, setModalVisible] = useState(false);
  const [selectedNotification, setSelectedNotification] = useState(null);
  const navigationRef = React.useRef(null);

  // Lifted state for reservations data
  const [availabilities, setAvailabilities] = useState({});
  const [reservationsLoading, setReservationsLoading] = useState(false);
  const hasFetchedReservations = useRef(false);

  const fetchReservations = useCallback(async (force = false) => {
    if (!force && hasFetchedReservations.current) {
      console.log('[App] Skipping fetch, using cached data');
      return;
    }

    console.log('[App] Fetching reservations data...');
    setReservationsLoading(true);
    const baseUrl = "https://xoi-lap-xuong.com";
    const apiUrl = `${baseUrl}/viva-padel/calendar`;

    try {
      const response = await fetch(apiUrl, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          "CF-Access-Client-Id": process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_ID,
          "CF-Access-Client-Secret": process.env.EXPO_PUBLIC_CF_ACCESS_CLIENT_SECRET,
        },
      });
      const responseText = await response.text();

      if (!response.ok) {
        Alert.alert("Server Error", "Could not fetch availabilities.");
        setReservationsLoading(false);
        return;
      }

      const data = JSON.parse(responseText);
      setAvailabilities(data.availabilities || {});
      hasFetchedReservations.current = true;
    } catch (error) {
      console.error(error);
      Alert.alert("Connection Error", "The server is not available.");
    } finally {
      setReservationsLoading(false);
    }
  }, []);

  const lastResponse = Notifications.useLastNotificationResponse();

  useEffect(() => {
    if (lastResponse && lastResponse.notification) {
      const { title, body } = lastResponse.notification.request.content;
      setSelectedNotification({ title, body });
      setModalVisible(true);
    }
  }, [lastResponse]);

  useEffect(() => {
    NotificationService.registerForPushNotificationsAsync();

    const cleanup = NotificationService.initListeners(
      (notification) => {
        console.log('Foreground notification:', notification.request.content.title);
      },
      (response) => {
        const { title, body } = response.notification.request.content;
        setSelectedNotification({ title, body });
        setModalVisible(true);
      }
    );

    return cleanup;
  }, []);

  const openDrawer = () => setDrawerVisible(true);
  const closeDrawer = () => setDrawerVisible(false);

  const onStateChange = async () => {
    const previousRouteName = currentScreen;
    const currentRouteName = navigationRef.current.getCurrentRoute().name;

    if (previousRouteName !== currentRouteName) {
      setCurrentScreen(currentRouteName);
    }
  };

  const navigateTo = (screenName) => {
    setCurrentScreen(screenName);
    if (navigationRef.current) {
      navigationRef.current.navigate(screenName);
    }
  };

  return (
    <SafeAreaProvider>
      <NavigationContainer ref={navigationRef} onStateChange={onStateChange}>
        <Stack.Navigator
          screenOptions={{
            headerShown: false,
            animation: 'fade',
            freezeOnBlur: true,
          }}
          detachInactiveScreens={false}
        >
          <Stack.Screen name="Home">
            {(props) => <HomeScreen {...props} openDrawer={openDrawer} />}
          </Stack.Screen>
          <Stack.Screen name="Reservations">
            {(props) => (
              <ReservationsScreen
                {...props}
                openDrawer={openDrawer}
                availabilities={availabilities}
                loading={reservationsLoading}
                onRefresh={() => fetchReservations(true)}
                onInitialLoad={() => fetchReservations(false)}
              />
            )}
          </Stack.Screen>
          <Stack.Screen name="Alarms">
            {(props) => <AlarmsScreen {...props} openDrawer={openDrawer} />}
          </Stack.Screen>
        </Stack.Navigator>
      </NavigationContainer>

      <CustomDrawer
        visible={drawerVisible}
        onClose={closeDrawer}
        onNavigate={navigateTo}
        currentScreen={currentScreen}
      />

      {/* Notification Content Modal */}
      <Modal
        animationType="fade"
        transparent={true}
        visible={modalVisible}
        onRequestClose={() => setModalVisible(false)}
      >
        <View style={styles.modalBg}>
          <View style={styles.modalContainer}>
            <Text style={styles.modalTitle}>{selectedNotification?.title || 'Notification'}</Text>
            <Text style={styles.modalBody}>{selectedNotification?.body}</Text>
            <TouchableOpacity style={styles.closeButton} onPress={() => setModalVisible(false)}>
              <Text style={styles.closeButtonText}>Fermer</Text>
            </TouchableOpacity>
          </View>
        </View>
      </Modal>
    </SafeAreaProvider>
  );
}

const styles = StyleSheet.create({
  modalBg: {
    flex: 1,
    backgroundColor: 'rgba(0,0,0,0.5)',
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20
  },
  modalContainer: {
    backgroundColor: 'white',
    borderRadius: 20,
    padding: 25,
    width: '90%',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.25,
    shadowRadius: 4,
    elevation: 5
  },
  modalTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    marginBottom: 10,
    color: '#333'
  },
  modalBody: {
    fontSize: 16,
    color: '#666',
    marginBottom: 20,
    lineHeight: 22
  },
  closeButton: {
    backgroundColor: '#007AFF',
    paddingVertical: 12,
    borderRadius: 10,
    alignItems: 'center'
  },
  closeButtonText: {
    color: 'white',
    fontSize: 16,
    fontWeight: '600'
  }
});
