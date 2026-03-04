import * as Notifications from "expo-notifications";
import * as Device from "expo-device";
import Constants from "expo-constants";
import { Platform } from "react-native";
import { storage } from "../utils/storage";
import { apiFetch } from "../utils/apiUtils";

const isWeb = Platform.OS === "web";
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const LAST_TOKEN_KEY = "viva_padel_last_notif_token";
const LAST_USER_KEY = "viva_padel_last_notif_user";
const INSTALLATION_ID_KEY = "viva_padel_installation_id";

/**
 * Generates a pseudo-random UUID v4.
 */
function generateUUID() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    const r = (Math.random() * 16) | 0;
    const v = c === "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

// This handler determines how to show a notification when the app is in the FOREGROUND.
if (!isWeb) {
  Notifications.setNotificationHandler({
    handleNotification: async () => ({
      shouldShowAlert: true,
      shouldPlaySound: true,
      shouldSetBadge: true,
    }),
  });
}

export const NotificationService = {
  /**
   * Gets or creates a persistent unique installation ID for this device.
   */
  getInstallationId: async () => {
    try {
      let id = await storage.getItem(INSTALLATION_ID_KEY);
      if (!id) {
        id = generateUUID();
        await storage.setItem(INSTALLATION_ID_KEY, id);
        console.log("[NotificationService] Generated new installation ID:", id);
      }
      return id;
    } catch (e) {
      console.error(
        "[NotificationService] Failed to get/set installation ID:",
        e,
      );
      return Device.osBuildId || "unknown_device";
    }
  },

  /**
   * Registers the device for push notifications and returns the token.
   */
  registerForPushNotificationsAsync: async () => {
    if (isWeb) {
      // For PWA applications
      if (!("serviceWorker" in navigator) || !("PushManager" in window)) {
        console.log(
          "[NotificationService] Web Push is not supported in this browser.",
        );
        return null;
      }

      try {
        let status = Notification.permission;
        if (status !== "granted") {
          status = await Notification.requestPermission();
        }

        if (status !== "granted") {
          console.error(
            "[NotificationService] Permission for web push not granted. Status:",
            status,
          );
          return null;
        }

        // Register service worker if not already registered
        const registration =
          await navigator.serviceWorker.register("/service_worker.js");
        await navigator.serviceWorker.ready;

        const applicationServerKey = process.env.EXPO_PUBLIC_VAPID_PUBLIC_KEY;

        if (!applicationServerKey) {
          console.warn(
            "[NotificationService] EXPO_PUBLIC_VAPID_PUBLIC_KEY is not defined in environment variables.",
          );
          return null;
        }

        // Convert the base64 string public VAPID key to a Uint8Array
        const urlBase64ToUint8Array = (base64String) => {
          const padding = "=".repeat((4 - (base64String.length % 4)) % 4);
          const base64 = (base64String + padding)
            .replace(/\-/g, "+")
            .replace(/_/g, "/");

          const rawData = window.atob(base64);
          const outputArray = new Uint8Array(rawData.length);
          for (let i = 0; i < rawData.length; ++i) {
            outputArray[i] = rawData.charCodeAt(i);
          }
          return outputArray;
        };

        const expectedKeyArray = urlBase64ToUint8Array(applicationServerKey);

        let subscription = await registration.pushManager.getSubscription();

        if (subscription) {
          let keyMatches = false;
          if (
            subscription.options &&
            subscription.options.applicationServerKey
          ) {
            const existingKeyArray = new Uint8Array(
              subscription.options.applicationServerKey,
            );
            if (existingKeyArray.length === expectedKeyArray.length) {
              keyMatches = existingKeyArray.every(
                (val, index) => val === expectedKeyArray[index],
              );
            }
          }
          if (!keyMatches) {
            console.log(
              "[NotificationService] VAPID key changed, unsubscribing old subscription.",
            );
            await subscription.unsubscribe();
            subscription = null;
          }
        }

        if (!subscription) {
          subscription = await registration.pushManager.subscribe({
            userVisibleOnly: true,
            applicationServerKey: expectedKeyArray,
          });
        }

        const token = JSON.stringify(subscription);
        if (__DEV__) {
          console.log("--- WEB PUSH SUBSCRIPTION ---");
          console.log(token);
          console.log("-----------------------------");
        }
        return token;
      } catch (error) {
        console.error(
          "[NotificationService] Error registering web push:",
          error,
        );
        return null;
      }
    }

    let token;

    // Detect if we are running in Expo Go
    const isExpoGo = Constants.appOwnership === "expo";
    if (isExpoGo) {
      console.log(
        "[NotificationService] Running in Expo Go. Will attempt to get Expo token for testing.",
      );
    }

    if (!Device.isDevice) {
      console.warn(
        "[NotificationService] Not a physical device. Registration might fail or return a simulation token.",
      );
    }

    try {
      const { status: existingStatus } =
        await Notifications.getPermissionsAsync();
      let finalStatus = existingStatus;

      if (existingStatus !== "granted") {
        const { status } = await Notifications.requestPermissionsAsync();
        finalStatus = status;
      }

      if (finalStatus !== "granted") {
        console.error(
          "[NotificationService] Permission for push notifications not granted. Status:",
          finalStatus,
        );
        return null;
      }

      const maxAttempts = 3;
      for (let attempt = 1; attempt <= maxAttempts; attempt++) {
        try {
          const projectId =
            Constants?.expoConfig?.extra?.eas?.projectId ??
            Constants?.easConfig?.projectId;

          if (!projectId) {
            console.error(
              "[NotificationService] Project ID not found in app config. Ensure EAS is configured.",
            );
            return null;
          }

          console.log(
            `[NotificationService] Attempting to fetch push token (Attempt ${attempt}/${maxAttempts})...`,
          );

          token = (
            await Notifications.getExpoPushTokenAsync({
              projectId: projectId,
            })
          ).data;

          if (token) {
            console.log("--- DEVICE PUSH TOKEN ---");
            console.log(token);
            console.log("-------------------------");
            break;
          }
        } catch (e) {
          console.error(
            `[NotificationService] Error fetching push token (Attempt ${attempt}):`,
            e,
          );
          if (attempt === maxAttempts) {
            try {
              console.error(
                "[NotificationService] Final registration error details:",
                JSON.stringify(e, null, 2),
              );
            } catch (err) {
              console.error(
                "[NotificationService] Could not stringify final error object.",
              );
            }
          } else {
            const waitTime = attempt * 1000;
            await delay(waitTime);
          }
        }
      }

      if (Platform.OS === "android") {
        Notifications.setNotificationChannelAsync("default", {
          name: "default",
          importance: Notifications.AndroidImportance.MAX,
          vibrationPattern: [0, 250, 250, 250],
          lightColor: "#FF231F7C",
        });
      }

      return token;
    } catch (error) {
      console.error(
        "[NotificationService] Error in registerForPushNotificationsAsync:",
        error,
      );
      return null;
    }
  },

  /**
   * Registers the device with the backend server.
   */
  registerDeviceWithServer: async (
    pushToken,
    userToken,
    userEmail,
    force = false,
  ) => {
    if (!pushToken || !userToken || !userEmail) {
      console.warn(
        "[NotificationService] Missing pushToken, userToken or userEmail for registration",
      );
      return;
    }

    if (!force) {
      try {
        const lastToken = await storage.getItem(LAST_TOKEN_KEY);
        const lastUser = await storage.getItem(LAST_USER_KEY);

        if (lastToken === pushToken && lastUser === userEmail) {
          if (__DEV__)
            console.log(
              "[NotificationService] Device registration matches cache. Skipping server call.",
            );
          return;
        }
      } catch (e) {
        console.warn(
          "[NotificationService] Failed to read registration cache:",
          e,
        );
      }
    }

    const deviceId = await NotificationService.getInstallationId();

    try {
      console.log("[NotificationService] Registering device with server...", {
        deviceId,
        pushToken,
        userEmail,
      });
      var response = null;
      if (isWeb) {
        const subscription = JSON.parse(pushToken);
        response = await apiFetch("/register-device", {
          method: "POST",
          headers: {
            "Internal-Auth": `Bearer ${userToken}`,
          },
          body: JSON.stringify({
            device_id: deviceId,
            notif_info: {
              Web: subscription,
            },
          }),
        });
      } else {
        response = await apiFetch("/register-device", {
          method: "POST",
          headers: {
            "Internal-Auth": `Bearer ${userToken}`,
          },
          body: JSON.stringify({
            device_id: deviceId,
            notif_info: {
              Mobile: {
                notif_token: pushToken,
              },
            },
          }),
        });
      }

      if (!response.ok) {
        const errorText = await response.text();
        console.error("[NotificationService] Registration failed:", errorText);
      } else {
        console.log("[NotificationService] Device registered successfully");
        try {
          await storage.setItem(LAST_TOKEN_KEY, pushToken);
          await storage.setItem(LAST_USER_KEY, userEmail);
        } catch (e) {
          console.warn(
            "[NotificationService] Failed to write registration cache:",
            e,
          );
        }
      }
    } catch (error) {
      console.error(
        "[NotificationService] Network error during registration:",
        error,
      );
    }
  },

  /**
   * Sets up listeners for notifications.
   */
  initListeners: (onReceived, onResponse) => {
    if (isWeb) return () => {};

    const notificationListener = Notifications.addNotificationReceivedListener(
      (notification) => {
        if (onReceived) onReceived(notification);
      },
    );

    const responseListener =
      Notifications.addNotificationResponseReceivedListener((response) => {
        if (onResponse) onResponse(response);
      });

    return () => {
      notificationListener.remove();
      responseListener.remove();
    };
  },
};
