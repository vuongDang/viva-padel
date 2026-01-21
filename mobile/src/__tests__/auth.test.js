import React from "react";
import { render, fireEvent, waitFor } from "@testing-library/react-native";
import LoginModal from "../components/Modals/LoginModal";
import { AuthService } from "../services/authService";

// Mock AuthService
jest.mock("../services/authService", () => ({
    AuthService: {
        login: jest.fn(),
        signup: jest.fn(),
    }
}));

describe("LoginModal", () => {
    const mockOnClose = jest.fn();
    const mockOnLogin = jest.fn();

    beforeEach(() => {
        jest.clearAllMocks();
    });

    test("renders correctly in login mode by default", () => {
        const { getByText, getByPlaceholderText } = render(
            <LoginModal visible={true} onClose={mockOnClose} onLogin={mockOnLogin} />
        );

        expect(getByText("Connexion")).toBeTruthy();
        expect(getByPlaceholderText("votre@email.com")).toBeTruthy();
        expect(getByText("Se connecter")).toBeTruthy();
    });

    test("toggles between login and signup modes", () => {
        const { getByText } = render(
            <LoginModal visible={true} onClose={mockOnClose} onLogin={mockOnLogin} />
        );

        const toggleButton = getByText("Pas de compte ? S'inscrire");
        fireEvent.press(toggleButton);

        expect(getByText("Inscription")).toBeTruthy();
        expect(getByText("Créer un compte")).toBeTruthy();
        expect(getByText("Déjà un compte ? Se connecter")).toBeTruthy();
    });

    test("calls login service on submit in login mode", async () => {
        AuthService.login.mockResolvedValue({ token: "test-token" });

        const { getByPlaceholderText, getByText } = render(
            <LoginModal visible={true} onClose={mockOnClose} onLogin={mockOnLogin} />
        );

        const emailInput = getByPlaceholderText("votre@email.com");
        fireEvent.changeText(emailInput, "test@example.com");

        const submitButton = getByText("Se connecter");
        fireEvent.press(submitButton);

        await waitFor(() => {
            expect(AuthService.login).toHaveBeenCalledWith("test@example.com");
            expect(mockOnLogin).toHaveBeenCalledWith("test@example.com", "test-token");
            expect(mockOnClose).toHaveBeenCalled();
        });
    });

    test("calls signup then login on submit in signup mode", async () => {
        AuthService.signup.mockResolvedValue({});
        AuthService.login.mockResolvedValue({ token: "test-token" });

        const { getByPlaceholderText, getByText } = render(
            <LoginModal visible={true} onClose={mockOnClose} onLogin={mockOnLogin} />
        );

        fireEvent.press(getByText("Pas de compte ? S'inscrire"));

        const emailInput = getByPlaceholderText("votre@email.com");
        fireEvent.changeText(emailInput, "new@example.com");

        fireEvent.press(getByText("Créer un compte"));

        await waitFor(() => {
            expect(AuthService.signup).toHaveBeenCalledWith("new@example.com");
            expect(AuthService.login).toHaveBeenCalledWith("new@example.com");
            expect(mockOnLogin).toHaveBeenCalledWith("new@example.com", "test-token");
        });
    });
});
