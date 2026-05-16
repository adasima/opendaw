// Wrapper for Tauri APIs to support browser-based testing with mocks

// Determine mode
const useMock = import.meta.env.MODE === 'mock' || import.meta.env.VITE_MOCK === 'true';

// Import mocks statically (they are small and only for dev)
// @ts-ignore
import * as mockTauri from "../mocks/tauri.ts";
// @ts-ignore
import * as mockDialog from "../mocks/dialog.ts";
// @ts-ignore
import * as mockWindow from "../mocks/window.ts";

// Real imports - we wrap these to avoid aggressive bundling issues or SSR crashes if possible
// But for named exports we need static imports usually.
// Depending on how tauri-apps works, direct import might be fine.
import { invoke as realInvoke } from "@tauri-apps/api/core";
import { listen as realListen } from "@tauri-apps/api/event";
import { open as realOpen, save as realSave, ask as realAsk } from "@tauri-apps/plugin-dialog";

// Invoke
export const invoke = useMock ? mockTauri.invoke : realInvoke;

// Event
export const listen = useMock ? mockTauri.listen : realListen;

// Dialog
export const open = useMock ? mockDialog.open : realOpen;
export const save = useMock ? mockDialog.save : realSave;
export const ask = useMock ? mockDialog.ask : realAsk;

// Window
// Helper to get window API safely (often lazy loaded)
export async function getWindowApi() {
    if (useMock) {
        return mockWindow;
    } else {
        return await import("@tauri-apps/api/window");
    }
}
