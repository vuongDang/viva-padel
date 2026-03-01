/// This is only called for PWA applications 


self.addEventListener('push', function (event) {
    if (!event.data) {
        return;
    }

    try {
        const data = event.data.json();

        const title = data.title || 'Viva Padel';
        const options = {
            body: data.body || '',
            icon: '/viva-padel-icon.png',
            badge: '/viva-padel-icon.png',
            data: data.data || {}
        };

        event.waitUntil(
            self.registration.showNotification(title, options)
        );
    } catch (e) {
        const title = 'Viva Padel';
        const options = {
            body: event.data.text()
        };
        event.waitUntil(
            self.registration.showNotification(title, options)
        );
    }
});

self.addEventListener('notificationclick', function (event) {
    event.notification.close();
    event.waitUntil(
        clients.matchAll({ type: 'window', includeUncontrolled: true }).then(function (clientList) {
            if (clientList.length > 0) {
                let client = clientList[0];
                for (let i = 0; i < clientList.length; i++) {
                    if (clientList[i].focused) {
                        client = clientList[i];
                    }
                }
                return client.focus();
            }
            return clients.openWindow('/');
        })
    );
});
