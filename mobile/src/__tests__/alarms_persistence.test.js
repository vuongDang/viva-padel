import React from "react";
import { render, fireEvent, waitFor } from "@testing-library/react-native";
import AlarmsScreen from "../screens/AlarmsScreen";
import { AlarmService } from "../services/alarmService";
import { Alert } from "react-native";

// Mock services
jest.mock("../services/alarmService", () => ({
    AlarmService: {
        syncAlarms: jest.fn(),
    }
}));

// Mock Alert
jest.spyOn(Alert, 'alert');

describe("AlarmsScreen Persistence", () => {
    const mockUser = { email: "test@example.com", token: "token" };
    const mockOpenDrawer = jest.fn();
    const mockOnLogin = jest.fn();
    const mockOnLogout = jest.fn();

    beforeEach(() => {
        jest.clearAllMocks();
    });

    test("loads alarms from serverAlarms prop on mount", () => {
        const serverAlarms = [
            { id: "1", name: "Morning Padel", weekdays: [0, 2, 4], startTime: "08:00", endTime: "10:00", types: { indoor: true, outdoor: true }, period: 1, activated: true }
        ];

        const { getByText } = render(
            <AlarmsScreen
                serverAlarms={serverAlarms}
                user={mockUser}
                openDrawer={mockOpenDrawer}
                onLogin={mockOnLogin}
                onLogout={mockOnLogout}
            />
        );

        expect(getByText("Morning Padel")).toBeTruthy();
    });

    test("calls AlarmService.syncAlarms when Activer is pressed", async () => {
        AlarmService.syncAlarms.mockResolvedValue(true);
        const serverAlarms = [
            { id: "1", name: "Morning Padel", weekdays: [0, 2, 4], startTime: "08:00", endTime: "10:00", types: { indoor: true, outdoor: true }, period: 1, activated: true }
        ];

        const { getByText } = render(
            <AlarmsScreen
                serverAlarms={serverAlarms}
                user={mockUser}
                openDrawer={mockOpenDrawer}
                onLogin={mockOnLogin}
                onLogout={mockOnLogout}
            />
        );

        const activateButton = getByText("Activer");
        fireEvent.press(activateButton);

        await waitFor(() => {
            expect(AlarmService.syncAlarms).toHaveBeenCalled();
            expect(Alert.alert).toHaveBeenCalledWith("Succès", expect.stringContaining("active sur le serveur"));
        });
    });

    test("prompts for login if user is not authenticated when clicking Activer", () => {
        const { getByText } = render(
            <AlarmsScreen
                serverAlarms={[]}
                user={null}
                openDrawer={mockOpenDrawer}
                onLogin={mockOnLogin}
                onLogout={mockOnLogout}
            />
        );

        const activateButton = getByText("Activer");
        fireEvent.press(activateButton);

        // Based on logic, it should show LoginModal (check by looking for "Connexion" title in modal if it's there, but we can just check if state would change or if LoginModal appears)
        // Since we can't easily check state, we look for modal header if it's rendered
        expect(getByText("Connexion")).toBeTruthy();
    });
});
