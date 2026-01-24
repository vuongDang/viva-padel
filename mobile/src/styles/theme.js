export const theme = {
    colors: {
        primary: '#1A1A1A',
        secondary: '#1A73E8',
        background: '#FAFAFA',
        card: '#FFFFFF',
        text: '#1A1A1A',
        textSecondary: '#666666',
        border: '#E0E0E0',
        divider: '#F0F0F0',
        error: '#FF4444',
        success: '#34A853',
        white: '#FFFFFF',

        // Calendar-specific colors (Restored)
        calendarPrimary: '#1a73e8',
        todayBg: '#373ddaff',
        available: '#34a853',
        unavailable: '#ea4335',
        availableLight: '#e6f4ea',
        unavailableLight: '#fce8e6',
    },

    shadows: {
        small: {
            shadowColor: '#000',
            shadowOffset: { width: 0, height: 2 },
            shadowOpacity: 0.1,
            shadowRadius: 4,
            elevation: 2,
        },
        medium: {
            shadowColor: '#000',
            shadowOffset: { width: 0, height: 4 },
            shadowOpacity: 0.3,
            shadowRadius: 4,
            elevation: 5,
        }
    },
    spacing: {
        xs: 4,
        s: 8,
        m: 16,
        l: 24,
        xl: 32,
    },
    styles: {
        header: {
            height: 56,
            flexDirection: 'row',
            alignItems: 'center',
            paddingHorizontal: 16,
            borderBottomWidth: 1,
            borderBottomColor: '#E0E0E0',
            backgroundColor: '#FFF',
        },
        floatingButtonContainer: {
            position: 'absolute',
            bottom: 30,
            left: 0,
            right: 0,
            flexDirection: 'row',
            justifyContent: 'center',
            alignItems: 'center',
            gap: 8,
            paddingHorizontal: 16,
            zIndex: 10,
        }
    }
};

