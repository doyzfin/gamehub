import { writable } from 'svelte/store';
import { load } from '@tauri-apps/plugin-store';

export interface Game {
    id: string;
    name: string;
    executable: string;
    workingDirectory?: string;
    cover?: string;
    background?: string;
    launchArguments?: string[];
    installed: boolean;
    lastPlayedAt?: string;
    totalPlaytimeSeconds: number;
    categories?: string[];
    favorite?: boolean;
}

function createGamesStore() {
    const { subscribe, set, update } = writable<Game[]>([]);
    let storeInstance: any = null;

    return {
        subscribe,
        init: async () => {
            try {
                storeInstance = await load('games.json', { autoSave: true });
                const savedGames = await storeInstance.get<Game[]>('library');
                if (savedGames) {
                    set(savedGames);
                } else {
                    await storeInstance.set('library', []);
                }
            } catch (e) {
                console.warn('Failed to load games store (might be in browser dev mode):', e);
            }
        },
        addGame: async (game: Game) => {
            update(games => {
                const newGames = [...games, game];
                if (storeInstance) storeInstance.set('library', newGames);
                return newGames;
            });
        },
        updateGame: async (id: string, updates: Partial<Game>) => {
            update(games => {
                const newGames = games.map(g => g.id === id ? { ...g, ...updates } : g);
                if (storeInstance) storeInstance.set('library', newGames);
                return newGames;
            });
        },
        removeGame: async (id: string) => {
            update(games => {
                const newGames = games.filter(g => g.id !== id);
                if (storeInstance) storeInstance.set('library', newGames);
                return newGames;
            });
        }
    };
}

export const games = createGamesStore();
