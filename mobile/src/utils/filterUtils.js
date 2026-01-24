export function matchesFilter(slot, playground, dateStr, currentAlarmId, alarms) {
    let alarm;
    if (currentAlarmId === 'all') {
        alarm = {
            types: { indoor: true, outdoor: true },
            weekdays: [0, 1, 2, 3, 4, 5, 6],
            startTime: '00:00',
            endTime: '23:59',
            slotDurations: [3600, 5400, 7200]
        };
    } else {
        alarm = alarms.find(a => a.id === currentAlarmId);
    }

    if (!alarm) return false;

    // 1. Type check
    if (playground.indoor && !alarm.types.indoor) return false;
    if (!playground.indoor && !alarm.types.outdoor) return false;

    // 2. Weekday check
    const date = new Date(dateStr);
    let dayOfWeek = date.getDay() - 1;
    if (dayOfWeek === -1) dayOfWeek = 6;
    if (!alarm.weekdays.includes(dayOfWeek)) return false;

    // 3. Time range check
    if (slot.startAt < alarm.startTime || slot.startAt > alarm.endTime) return false;

    // 4. Duration check
    const allowedDurations = alarm.slotDurations || [3600, 5400, 7200];
    return slot.prices.some(price => price.bookable && allowedDurations.includes(price.duration));
}


