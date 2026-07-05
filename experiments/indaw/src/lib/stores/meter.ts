import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

export interface MeterData {
    master_peak: [number, number];
    track_peaks: Record<number, [number, number]>;
}

const initialState: MeterData = {
    master_peak: [0, 0],
    track_peaks: {}
};

function createMeterStore() {
    const { subscribe, set } = writable<MeterData>(initialState);

    // Setup listener
    let unlisten: (() => void) | null = null;

    const mockEnabled = writable(false);

    async function init() {
        if (unlisten) return;

        // Try to listen to Tauri event first, fallback to mock if fails
        try {
            unlisten = await listen<MeterData>('meter-update', (event) => {
                set(event.payload);
            });
            console.log("MeterStore: Tauri listener connected.");
        } catch (e) {
            console.warn("MeterStore: Tauri listener failed, falling back to Mock Mode.", e);

            console.log("Starting Mock Metering (Paused by default)");
            // Mock Mode: Simulate metering
            const interval = setInterval(() => {
                let enabled = false;
                const unsub = mockEnabled.subscribe(v => enabled = v);
                unsub();

                if (!enabled) {
                    set({ master_peak: [0, 0], track_peaks: {} });
                    return;
                }

                const time = Date.now() / 200;
                const tracks: Record<number, [number, number]> = {};
                // Generate for potential 32 tracks
                for (let i = 0; i < 32; i++) {
                    tracks[i] = [Math.random() * 0.8, Math.random() * 0.8];
                }

                set({
                    master_peak: [
                        Math.abs(Math.sin(time)) * 0.8,
                        Math.abs(Math.cos(time)) * 0.8
                    ],
                    track_peaks: tracks
                });
            }, 50); // 20fps

            unlisten = () => clearInterval(interval);
        }
    }

    return {
        subscribe,
        init,
        mockEnabled
    };
}

export const meterStore = createMeterStore();
