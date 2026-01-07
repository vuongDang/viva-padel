import React, { useMemo } from 'react';
import { StyleSheet, View, Text, Modal, TouchableOpacity, FlatList, Pressable } from 'react-native';
import { theme } from '../../styles/theme';

const CourtItem = React.memo(({ playground, slotsByTime, onSlotClick }) => {
    const timePoints = useMemo(() => Object.keys(slotsByTime).sort(), [slotsByTime]);

    if (timePoints.length === 0) return null;

    return (
        <View style={styles.courtItem}>
            <Text style={styles.courtName}>{playground.name}</Text>
            <View style={styles.slotList}>
                {timePoints.map(time => {
                    const slotGroup = slotsByTime[time];
                    return (
                        <TouchableOpacity
                            key={time}
                            style={styles.slotTag}
                            onPress={() => onSlotClick(slotGroup)}
                        >
                            <Text style={styles.slotText}>{time}</Text>
                        </TouchableOpacity>
                    );
                })}
            </View>
        </View>
    );
});

export default function AvailabilityModal({ visible, dateStr, dayAvail, onClose, onSlotClick, filterFn }) {
    const formattedDate = useMemo(() => {
        if (!dateStr) return '';
        return new Date(dateStr).toLocaleDateString('fr-FR', {
            weekday: 'long', month: 'long', day: 'numeric'
        });
    }, [dateStr]);

    const chartData = useMemo(() => {
        if (!dayAvail || !dayAvail['hydra:member']) return [];

        return dayAvail['hydra:member'].map((playground, pIndex) => {
            const slotsByTime = {};
            playground.activities.forEach(activity => {
                activity.slots.forEach(slot => {
                    if (filterFn(slot, playground)) {
                        if (!slotsByTime[slot.startAt]) {
                            slotsByTime[slot.startAt] = {
                                startAt: slot.startAt,
                                durations: new Set(),
                                courtName: playground.name,
                                slotData: slot
                            };
                        }
                        slot.prices.forEach(price => {
                            if (price.bookable && price.duration >= 5400) {
                                slotsByTime[slot.startAt].durations.add(price.duration);
                            }
                        });
                    }
                });
            });

            return {
                id: playground.id || pIndex.toString(),
                playground,
                slotsByTime,
                hasSlots: Object.keys(slotsByTime).length > 0
            };
        }).filter(item => item.hasSlots);
    }, [dayAvail, filterFn]);

    if (!visible) return null;

    const renderItem = ({ item }) => (
        <CourtItem
            playground={item.playground}
            slotsByTime={item.slotsByTime}
            onSlotClick={onSlotClick}
        />
    );

    return (
        <Modal animationType="slide" transparent={true} visible={visible} onRequestClose={onClose}>
            <View style={styles.overlay}>
                <Pressable style={styles.background} onPress={onClose} />
                <View style={styles.content}>
                    <TouchableOpacity style={styles.closeBtn} onPress={onClose}>
                        <Text style={styles.closeText}>&times;</Text>
                    </TouchableOpacity>

                    <Text style={styles.title}>Disponibilités pour le {formattedDate}</Text>

                    {chartData.length === 0 ? (
                        <Text style={styles.emptyText}>Aucun créneau disponible</Text>
                    ) : (
                        <FlatList
                            data={chartData}
                            renderItem={renderItem}
                            keyExtractor={item => item.id}
                            initialNumToRender={5}
                            windowSize={5}
                            removeClippedSubviews={true}
                        />
                    )}
                </View>
            </View>
        </Modal>
    );
}

const styles = StyleSheet.create({
    overlay: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
        backgroundColor: 'rgba(0,0,0,0.5)',
    },
    background: {
        ...StyleSheet.absoluteFillObject,
    },
    content: {
        backgroundColor: 'white',
        width: '90%',
        maxHeight: '80%',
        borderRadius: 12,
        padding: 24,
        shadowColor: "#000",
        shadowOffset: { width: 0, height: 2 },
        shadowOpacity: 0.25,
        shadowRadius: 3.84,
        elevation: 5,
        // Ensure content doesn't catch touches for everything, but also doesn't pass them to background Pressable
        zIndex: 1,
    },
    closeBtn: {
        position: 'absolute',
        paddingHorizontal: 14,
        top: 0,
        right: 0,
        zIndex: 2,
    },
    closeText: {
        fontSize: 40,
        color: '#70757a',
    },
    title: {
        fontSize: 18,
        fontWeight: '600',
        marginBottom: 20,
        marginRight: 20,
    },
    courtItem: {
        borderWidth: 1,
        borderColor: theme.colors.border,
        borderRadius: 8,
        padding: 12,
        marginBottom: 12,
    },
    courtName: {
        fontWeight: '600',
        marginBottom: 8,
        color: theme.colors.primary,
    },
    slotList: {
        flexDirection: 'row',
        flexWrap: 'wrap',
        gap: 8,
    },
    slotTag: {
        backgroundColor: theme.colors.availableLight,
        paddingVertical: 6,
        paddingHorizontal: 10,
        borderRadius: 4,
        borderWidth: 1,
        borderColor: '#ceead6',
    },
    slotText: {
        color: '#137333',
        fontSize: 13,
    },
    emptyText: {
        textAlign: 'center',
        marginTop: 20,
        color: '#70757a'
    }
});

