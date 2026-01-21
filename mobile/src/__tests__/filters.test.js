import { matchesFilter } from "../utils/filterUtils";

describe("matchesFilter", () => {
    const mockPlaygroundIndoor = { indoor: true };
    const mockPlaygroundOutdoor = { indoor: false };

    const mockSlot = {
        startAt: "18:00",
        prices: [{ bookable: true, duration: 5400 }] // 90 min
    };

    const dateStr = "2026-01-20"; // Tuesday (index 1)

    const customFilters = [
        {
            id: "filter-1",
            types: { indoor: true, outdoor: false },
            weekdays: [1], // Tuesday
            startTime: "17:00",
            endTime: "20:00"
        }
    ];

    test("should match correctly with 'all' filter", () => {
        const result = matchesFilter(mockSlot, mockPlaygroundIndoor, dateStr, 'all', []);
        expect(result).toBe(true);
    });

    test("should match correct custom filter", () => {
        const result = matchesFilter(mockSlot, mockPlaygroundIndoor, dateStr, 'filter-1', customFilters);
        expect(result).toBe(true);
    });

    test("should fail if playground type mismatch", () => {
        const result = matchesFilter(mockSlot, mockPlaygroundOutdoor, dateStr, 'filter-1', customFilters);
        expect(result).toBe(false);
    });

    test("should fail if weekday mismatch", () => {
        const wednesday = "2026-01-21";
        const result = matchesFilter(mockSlot, mockPlaygroundIndoor, wednesday, 'filter-1', customFilters);
        expect(result).toBe(false);
    });

    test("should fail if time range mismatch", () => {
        const lateSlot = { ...mockSlot, startAt: "21:00" };
        const result = matchesFilter(lateSlot, mockPlaygroundIndoor, dateStr, 'filter-1', customFilters);
        expect(result).toBe(false);
    });

    test("should fail if no bookable price of 90min", () => {
        const shortSlot = {
            ...mockSlot,
            prices: [{ bookable: true, duration: 3600 }] // 60 min
        };
        const result = matchesFilter(shortSlot, mockPlaygroundIndoor, dateStr, 'filter-1', customFilters);
        expect(result).toBe(false);
    });
});
