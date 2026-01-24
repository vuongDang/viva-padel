export function matchesFilter(slot, playground, dateStr, currentFilterId, customFilters) {
    let filter;
    if (currentFilterId === 'all') {
        filter = {
            types: { indoor: true, outdoor: true },
            weekdays: [0, 1, 2, 3, 4, 5, 6],
            startTime: '00:00',
            endTime: '23:59',
            slotDurations: [3600, 5400, 7200]
        };
    } else {
        filter = customFilters.find(f => f.id === currentFilterId);
    }

    if (!filter) return false;

    // 1. Type check
    if (playground.indoor && !filter.types.indoor) return false;
    if (!playground.indoor && !filter.types.outdoor) return false;

    // 2. Weekday check
    const date = new Date(dateStr);
    let dayOfWeek = date.getDay() - 1;
    if (dayOfWeek === -1) dayOfWeek = 6;
    if (!filter.weekdays.includes(dayOfWeek)) return false;

    // 3. Time range check
    if (slot.startAt < filter.startTime || slot.startAt > filter.endTime) return false;

    // 4. Duration check
    const allowedDurations = filter.slotDurations || [3600, 5400, 7200];
    return slot.prices.some(price => price.bookable && allowedDurations.includes(price.duration));
}

