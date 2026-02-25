import React from 'react';
import { Text, StyleSheet } from 'react-native';

const RefreshNote = ({ timestamp, style }) => {
    return (
        <Text style={[styles.footerNote, style]}>
            Les disponibilités sont rafraîchies toutes les 30 min de 7:00 à 23:00.{"\n"}
            {timestamp ? `Dernière mise à jour : ${new Date(timestamp * 1000).toLocaleString('fr-FR', { hour: '2-digit', minute: '2-digit' })}` : "Chargement..."}
        </Text>
    );
};

const styles = StyleSheet.create({
    footerNote: {
        marginTop: 20,
        textAlign: 'center',
        fontSize: 12,
        color: '#999',
        fontStyle: 'italic',
        lineHeight: 18,
    },
});

export default RefreshNote;
