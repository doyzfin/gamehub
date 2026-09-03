import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';

export interface Settings {
    startWithWindows: boolean;
    exitHoldDuration: number;
    theme: 'dark' | 'midnight' | 'oled';
    scanDirectories: string[];
}

const defaultSettings: Settings = {
    startWithWindows: false,
    exitHoldDuration: 5000,
    theme: 'dark',
    scanDirectories: ['C:\\Games', 'C:\\Program Files (x86)\\Steam\\steamapps\\common', 'D:\\SteamLibrary']
};

function createSettingsStore() {
    const { subscribe, set, update } = writable<Settings>(defaultSettings);
    let storeInstance: any = null;

    return {
        subscribe,
        init: async () => {
            try {
                storeInstance = await load('settings.json', { autoSave: true });
                const savedSettings = (await storeInstance.get('preferences')) as Settings;
                
                let currentSettings = defaultSettings;
                if (savedSettings) {
                    currentSettings = { ...defaultSettings, ...savedSettings };
                } else {
                    await storeInstance.set('preferences', defaultSettings);
                }
                
                // Sync autostart state
                try {
                    const autostartEnabled = await isEnabled();
                    currentSettings.startWithWindows = autostartEnabled;
                } catch (e) {
                    console.warn('Autostart plugin not available in this environment');
                }
                
                set(currentSettings);
            } catch (e) {
                console.warn('Failed to load settings store:', e);
            }
        },
        updateSetting: async <K extends keyof Settings>(key: K, value: Settings[K]) => {
            update(settings => {
                const newSettings = { ...settings, [key]: value };
                if (storeInstance) storeInstance.set('preferences', newSettings);
                
                // Special handling for autostart
                if (key === 'startWithWindows') {
                    if (value) {
                        enable().catch(console.error);
                    } else {
                        disable().catch(console.error);
                    }
                }
                
                return newSettings;
            });
        }
    };
}

export const settings = createSettingsStore();
