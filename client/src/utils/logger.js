/**
 * Simple logger utility to intercept console methods and store logs in memory.
 * Useful for debugging production/preview builds where remote debugging is not available.
 */

let logs = [];
const MAX_LOGS = 200;
let listeners = new Set();

const originalConsole = {
    log: console.log,
    warn: console.warn,
    error: console.error,
};

const addLog = (type, args) => {
    const timestamp = new Date().toLocaleTimeString();
    const message = args
        .map(arg => {
            if (typeof arg === 'object') {
                try {
                    return JSON.stringify(arg, null, 2);
                } catch (e) {
                    return '[Unserializable Object]';
                }
            }
            return String(arg);
        })
        .join(' ');

    const newLog = { id: Date.now() + Math.random(), type, timestamp, message };

    logs = [newLog, ...logs].slice(0, MAX_LOGS);

    // Notify components that we have new logs
    listeners.forEach(listener => listener([...logs]));

    // Still output to the real console
    originalConsole[type](...args);
};

export const Logger = {
    init: () => {
        console.log = (...args) => addLog('log', args);
        console.warn = (...args) => addLog('warn', args);
        console.error = (...args) => addLog('error', args);
        console.log('[Logger] console interception initialized');
    },

    getLogs: () => logs,

    clear: () => {
        logs = [];
        listeners.forEach(listener => listener([]));
    },

    subscribe: (callback) => {
        listeners.add(callback);
        callback([...logs]);
        return () => listeners.delete(callback);
    },
};
