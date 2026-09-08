// defines: main.preferences.selection persistent preference contract
                const key = 'main.preferences.v1';
                export function load() { try { return JSON.parse(localStorage.getItem(key)) || {}; } catch { return {}; } }
                export function save(value) { try { localStorage.setItem(key, JSON.stringify(value)); } catch {} }
