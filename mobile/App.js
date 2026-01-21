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
import { AuthService } from './src/services/authService';
import { AlarmService } from './src/services/alarmService';

const Stack = createNativeStackNavigator();

export default function App() {
  const [drawerVisible, setDrawerVisible] = useState(false);
  const [currentScreen, setCurrentScreen] = useState('Home');
  const [modalVisible, setModalVisible] = useState(false);
  const [selectedNotification, setSelectedNotification] = useState(null);
  const navigationRef = React.useRef(null);

  // Authentication state
  const [user, setUser] = useState(null);
  const [serverAlarms, setServerAlarms] = useState([]);

  // Lifted state for reservations data
  const [availabilities, setAvailabilities] = useState({});
  const [calendarTimestamp, setCalendarTimestamp] = useState(null);
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
      setCalendarTimestamp(data.timestamp || null);
      hasFetchedReservations.current = true;
    } catch (error) {
      console.error(error);
      Alert.alert("Connection Error", "The server is not available.");
    } finally {
      setReservationsLoading(false);
    }
  }, []);

  const fetchUserInfo = useCallback(async (token, email) => {
    try {
      const data = await AuthService.getUserInfo(token);
      if (data.alarms) {
        const mapped = AlarmService.mapServerAlarmsToMobile(data.alarms);
        setServerAlarms(mapped);
        console.log('[App] Fetched and mapped alarms:', mapped.length);
      }
    } catch (error) {
      console.error('[App] Failed to fetch user info:', error);
      // If user info fails but we have token/email, we still stay logged in
      // but alarms won't be synced.
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
    // Check for existing user session on mount
    const checkUser = async () => {
      const token = await AuthService.getToken();
      const email = await AuthService.getEmail();
      if (token && email) {
        setUser({ email, token });
        console.log('[App] Authenticated user found:', email);
        fetchUserInfo(token, email);
      }
    };
    checkUser();

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
  }, [fetchUserInfo]);

  // Handle push token registration/updates
  useEffect(() => {
    if (!user) return;

    let isMounted = true;

    const register = async () => {
      console.log('[App] Attempting to register device for user:', user.email);
      const token = await NotificationService.registerForPushNotificationsAsync();
      console.log('[App] Received push token:', token ? 'YES' : 'NONE');

      if (token && user && isMounted) {
        await NotificationService.registerDeviceWithServer(token, user.token, user.email);
      } else if (!token) {
        console.warn('[App] Could not register device: No push token received.');
      }
    };

    register();

    // Listen for token changes
    const subscription = Notifications.addPushTokenListener(async (token) => {
      console.log('[App] Push token changed:', token.data);
      if (user && isMounted) {
        await NotificationService.registerDeviceWithServer(token.data, user.token, user.email);
      }
    });

    return () => {
      isMounted = false;
      subscription.remove();
    };
  }, [user]);

  const handleLogin = (email, token) => {
    setUser({ email, token });
    fetchUserInfo(token, email);
  };

  const handleLogout = async () => {
    await AuthService.logout();
    setUser(null);
    setServerAlarms([]);
  };

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
            {(props) => (
              <HomeScreen
                {...props}
                openDrawer={openDrawer}
                user={user}
                onLogin={handleLogin}
                onLogout={handleLogout}
              />
            )}
          </Stack.Screen>
          <Stack.Screen name="Reservations">
            {(props) => (
              <ReservationsScreen
                {...props}
                openDrawer={openDrawer}
                availabilities={availabilities}
                calendarTimestamp={calendarTimestamp}
                loading={reservationsLoading}
                onRefresh={() => fetchReservations(true)}
                onInitialLoad={() => fetchReservations(false)}
                user={user}
              />
            )}
          </Stack.Screen>
          <Stack.Screen name="Alarms">
            {(props) => (
              <AlarmsScreen
                {...props}
                openDrawer={openDrawer}
                user={user}
                serverAlarms={serverAlarms}
                onLogin={handleLogin}
                onLogout={handleLogout}
              />
            )}
          </Stack.Screen>
        </Stack.Navigator>
      </NavigationContainer>

      <CustomDrawer
        visible={drawerVisible}
        onClose={closeDrawer}
        onNavigate={navigateTo}
        currentScreen={currentScreen}
        user={user}
        onLogout={handleLogout}
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
