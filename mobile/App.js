import React, { useEffect, useState, useCallback, useRef, useMemo } from 'react';
import { Modal, View, Text, StyleSheet, TouchableOpacity, Alert } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import HomeScreen from './src/screens/HomeScreen';
import CalendarScreen from './src/screens/CalendarScreen';
import TimeSlotsScreen from './src/screens/TimeSlotsScreen';

import CustomDrawer from './src/components/CustomDrawer';
import * as Notifications from 'expo-notifications';
import * as Updates from 'expo-updates';
import { fetchWithTimeout } from './src/utils/apiUtils';
import { NotificationService } from './src/services/notificationService';
import { AuthService } from './src/services/authService';
import { AlarmService } from './src/services/alarmService';
import { matchesFilter } from './src/utils/filterUtils';

const Stack = createNativeStackNavigator();

export default function App() {
  const [drawerVisible, setDrawerVisible] = useState(false);
  const [currentScreen, setCurrentScreen] = useState('Home');
  const [modalVisible, setModalVisible] = useState(false);
  const [selectedNotification, setSelectedNotification] = useState(null);
  const navigationRef = React.useRef(null);

  // Authentication state
  const [user, setUser] = useState(null);
  const [alarms, setAlarms] = useState([]);
  const [matchedResults, setMatchedResults] = useState({});

  useEffect(() => {
    async function updateListener() {
      try {
        const update = await Updates.checkForUpdateAsync();
        if (update.isAvailable) {
          await Updates.fetchUpdateAsync();
          Alert.alert(
            "Mise à jour disponible",
            "Une nouvelle version de l'application est prête. Redémarrer maintenant pour l'utiliser ?",
            [
              { text: "Plus tard", style: "cancel" },
              { text: "Redémarrer", onPress: () => Updates.reloadAsync() }
            ]
          );
        }
      } catch (error) {
        console.log('[App] Error checking for updates:', error);
      }
    }

    if (!__DEV__) {
      updateListener();
    }
  }, []);

  // Lifted state for reservations data
  const [availabilities, setAvailabilities] = useState({});
  const [calendarTimestamp, setCalendarTimestamp] = useState(null);
  const [reservationsLoading, setReservationsLoading] = useState(false);
  const hasFetchedReservations = useRef(false);
  const isInitialAlarmsLoaded = useRef(false);

  // Global filtered matches shared across screens
  const filteredMatches = useMemo(() => {
    if (!availabilities) return { all: {} };

    const matches = { all: {} };

    // 1. Calculate matches for "all" (Tous)
    Object.entries(availabilities).forEach(([dateStr, dayAvail]) => {
      if (!dayAvail?.["hydra:member"]) return;
      const hasMatch = dayAvail["hydra:member"].some(playground =>
        playground.activities.some(activity =>
          activity.slots.some(slot => matchesFilter(slot, playground, dateStr, 'all', []))
        )
      );
      if (hasMatch) {
        matches.all[dateStr] = dayAvail;
      }
    });

    // 2. Calculate matches for each alarm
    alarms.forEach(alarm => {
      matches[alarm.id] = {};
      Object.entries(availabilities).forEach(([dateStr, dayAvail]) => {
        if (!dayAvail?.["hydra:member"]) return;
        const hasMatch = dayAvail["hydra:member"].some(playground =>
          playground.activities.some(activity =>
            activity.slots.some(slot => matchesFilter(slot, playground, dateStr, alarm.id, alarms))
          )
        );
        if (hasMatch) {
          matches[alarm.id][dateStr] = dayAvail;
        }
      });
    });

    return matches;
  }, [availabilities, alarms]);

  const fetchReservations = useCallback(async (force = false) => {
    if (!force && hasFetchedReservations.current) {
      console.log('[App] Skipping fetch, using cached data');
      return;
    }

    console.log('[App] Fetching reservations data...');
    setReservationsLoading(true);
    const apiUrl = `${process.env.EXPO_PUBLIC_API_URL}/calendar`;

    try {
      const response = await fetchWithTimeout(apiUrl, {
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
        const serverMapped = AlarmService.mapServerAlarmsToMobile(data.alarms);

        setAlarms(prevAlarms => {
          // Merging logic: Server alarms take precedence on name match
          // We use a Map to handle duplicates by name easily
          const alarmMap = new Map();

          // 1. Load local ones first
          prevAlarms.forEach(a => alarmMap.set(a.name, a));

          // 2. Overwrite or add with server ones
          serverMapped.forEach(sa => {
            alarmMap.set(sa.name, sa);
          });

          const merged = Array.from(alarmMap.values());
          console.log('[App] Merged alarms with server precedence. Total:', merged.length);
          return merged;
        });
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
      const { data } = lastResponse.notification.request.content;

      if (data?.availabilities) {
        handleIncomingMatchedResults(data.availabilities);
        navigateTo('TimeSlots');
      } else {

        const { title, body } = lastResponse.notification.request.content;
        setSelectedNotification({ title, body });
        setModalVisible(true);
      }
    }
  }, [lastResponse, handleIncomingMatchedResults, navigateTo]);

  useEffect(() => {
    const setup = async () => {
      // 1. Load local data for immediate UI
      const [localAlarms, localResults] = await Promise.all([
        AlarmService.getLocalAlarms(),
        AlarmService.getMatchedResults()
      ]);

      if (localAlarms?.length > 0) setAlarms(localAlarms);
      if (localResults) setMatchedResults(localResults);

      // Signal that we've finished the initial load from storage
      isInitialAlarmsLoaded.current = true;

      // 2. Check for existing session
      const token = await AuthService.getToken();
      const email = await AuthService.getEmail();
      if (token && email) {
        setUser({ email, token });
        console.log('[App] Authenticated user found:', email);
        fetchUserInfo(token, email);
      }

      // 3. Pre-fetch reservations for matching in TimeSlotsScreen
      fetchReservations(false);
    };

    setup();

    const cleanup = NotificationService.initListeners(
      (notification) => {
        console.log('Foreground notification:', notification.request.content.title);
      },
      (response) => {
        const { data } = response.notification.request.content;

        if (data?.availabilities) {
          handleIncomingMatchedResults(data.availabilities);
          navigateTo('TimeSlots');
        } else {

          const { title, body } = response.notification.request.content;
          setSelectedNotification({ title, body });
          setModalVisible(true);
        }
      }
    );

    return cleanup;
  }, [fetchUserInfo]);

  // Centralized Auto-Save for Alarms
  useEffect(() => {
    if (isInitialAlarmsLoaded.current) {
      console.log('[App] Auto-saving alarms to local storage...', alarms.length);
      AlarmService.saveLocalAlarms(alarms);
    }
  }, [alarms]);

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
    setAlarms([]);
    setMatchedResults({});
    await AlarmService.saveMatchedResults({});
  };

  const openDrawer = () => setDrawerVisible(true);
  const closeDrawer = () => setDrawerVisible(false);

  const onStateChange = async () => {
    if (!navigationRef.current) return;
    const currentRouteName = navigationRef.current.getCurrentRoute().name;

    if (currentScreen !== currentRouteName) {
      setCurrentScreen(currentRouteName);
    }
  };

  const navigateTo = (screenName, params = {}) => {
    setCurrentScreen(screenName);
    if (navigationRef.current) {
      navigationRef.current.navigate(screenName, params);
    }
  };

  const handleUpdateAlarms = async (newAlarms) => {
    setAlarms(newAlarms);
  };


  const handleSaveAlarm = async (alarmConfig) => {
    let finalName = alarmConfig.name;
    const otherAlarms = alarms.filter(a => a.id !== alarmConfig.id);

    let counter = 1;
    while (otherAlarms.some(a => a.name === finalName)) {
      finalName = `${alarmConfig.name} ${counter}`;
      counter++;
    }

    const configWithUniqueName = { ...alarmConfig, name: finalName };
    let finalAlarm;
    let updatedAlarms;


    if (alarmConfig.id) {
      finalAlarm = configWithUniqueName;
      updatedAlarms = alarms.map(a => a.id === alarmConfig.id ? finalAlarm : a);
    } else {
      finalAlarm = {
        ...configWithUniqueName,
        id: Date.now().toString(),
        activated: true
      };
      updatedAlarms = [...alarms, finalAlarm];
    }
    handleUpdateAlarms(updatedAlarms);
    return finalAlarm;
  };


  const handleDeleteAlarm = (id) => {
    const updatedAlarms = alarms.filter(alarm => alarm.id !== id);
    handleUpdateAlarms(updatedAlarms);
  };

  const handleToggleAlarm = (id) => {
    const updatedAlarms = alarms.map(alarm =>
      alarm.id === id ? { ...alarm, activated: !alarm.activated } : alarm
    );
    handleUpdateAlarms(updatedAlarms);
  };


  const handleIncomingMatchedResults = useCallback(async (newResults) => {
    setMatchedResults(prev => {
      const updated = { ...prev };
      // Deep merge new results into existing ones
      Object.entries(newResults).forEach(([alarmName, days]) => {
        updated[alarmName] = { ...(updated[alarmName] || {}), ...days };
      });
      AlarmService.saveMatchedResults(updated);
      return updated;
    });
  }, []);

  const handleClearMatchedResult = async (alarmName) => {
    setMatchedResults(prev => {
      const updated = { ...prev };
      delete updated[alarmName];
      AlarmService.saveMatchedResults(updated);
      return updated;
    });
  };

  const handleSyncAlarms = async (alarmsToSync, weeksAhead) => {
    if (!user) throw new Error("Veuillez vous connecter pour synchroniser vos créneaux.");
    return await AlarmService.syncAlarms(alarmsToSync || alarms, weeksAhead);
  };



  return (
    <SafeAreaProvider>
      <NavigationContainer ref={navigationRef} onStateChange={onStateChange}>
        <Stack.Navigator
          screenOptions={{
            headerShown: false,
            animation: 'slide_from_right',
            freezeOnBlur: true,
          }}
          detachInactiveScreens={true}
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
          <Stack.Screen name="Calendar">
            {(props) => (
              <CalendarScreen
                {...props}
                openDrawer={openDrawer}
                availabilities={availabilities}
                filteredMatches={filteredMatches}
                calendarTimestamp={calendarTimestamp}
                loading={reservationsLoading}
                onRefresh={() => fetchReservations(true)}
                onInitialLoad={() => fetchReservations(false)}
                user={user}
                onLogin={handleLogin}
                onLogout={handleLogout}
                alarms={alarms}
                onSaveAlarm={handleSaveAlarm}
                onDeleteAlarm={handleDeleteAlarm}
              />
            )}
          </Stack.Screen>
          <Stack.Screen name="TimeSlots">
            {(props) => (
              <TimeSlotsScreen
                {...props}
                openDrawer={openDrawer}
                user={user}
                alarms={alarms}
                availabilities={availabilities}
                filteredMatches={filteredMatches}
                calendarTimestamp={calendarTimestamp}
                matchedResults={matchedResults}
                onRefresh={() => fetchReservations(true)}
                loading={reservationsLoading}
                onSaveAlarm={handleSaveAlarm}
                onDeleteAlarm={handleDeleteAlarm}
                onToggleAlarm={handleToggleAlarm}
                onClearMatchedResult={handleClearMatchedResult}
                onSync={handleSyncAlarms}
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
        onLogin={handleLogin}
        onSimulateMatch={handleIncomingMatchedResults}
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
