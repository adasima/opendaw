import { writable, type Writable, derived, get } from 'svelte/store';
import en from '../locales/en';
import ja from '../locales/ja';
// import { loadCustomLocale } from '$lib/api'; // Future implementation

// Define Dictionary type safely
type Dictionary = typeof en;
type DeepPartial<T> = {
    [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

const builtinLocales: Record<string, Dictionary> = {
    en,
    ja
};

import { invoke } from '../api';

// Simple Deep Merge
function deepMerge(target: any, source: any) {
    if (!source) return target;
    const output = { ...target };
    if (typeof target === 'object' && typeof source === 'object') {
        Object.keys(source).forEach(key => {
            if (typeof source[key] === 'object') {
                if (!(key in target)) Object.assign(output, { [key]: source[key] });
                else output[key] = deepMerge(target[key], source[key]);
            } else {
                Object.assign(output, { [key]: source[key] });
            }
        });
    }
    return output;
}

function createI18n() {
    // Initial locale from localStorage or default 'en' (or 'ja' if system? let's default to ja for user)
    const storedLocale = localStorage.getItem('indaw_locale');
    const systemLang = navigator.language.startsWith('ja') ? 'ja' : 'en';
    const initialLocale = storedLocale || systemLang;

    const locale: Writable<string> = writable(initialLocale);
    const customDictionary: Writable<DeepPartial<Dictionary> | null> = writable(null);

    const { subscribe, set, update } = locale;

    return {
        subscribe,
        customDictionary,
        set: (l: string) => {
            localStorage.setItem('indaw_locale', l);
            set(l);
        },
        update,
        loadCustom: async () => {
            try {
                const custom = await invoke('load_custom_locale');
                customDictionary.set(custom as any);
            } catch (e) {
                console.error("Failed to load custom locale:", e);
            }
        },
        openFolder: async () => {
            await invoke('open_locales_folder');
        }
    };
}

export const locale = createI18n();

// Derived store for the translation function
// Derived store for the translation function
// Derived store for the translation function
export const t = derived(
    [locale, locale.customDictionary],
    ([$locale, $customDictionary]) => {
        return (key: string, vars: Record<string, any> = {}) => {
            // Determine dictionary
            let dict: any = builtinLocales[$locale] || builtinLocales['en'];

            if ($locale === 'custom' && $customDictionary) {
                dict = deepMerge(builtinLocales['ja'], $customDictionary);
            }

            // Split key (e.g., "menu.file")
            const keys = key.split('.');
            let value: any = dict;

            for (const k of keys) {
                if (value && typeof value === 'object' && k in value) {
                    value = value[k];
                } else {
                    value = null;
                    break;
                }
            }

            // Fallback to EN if missing
            if (!value && $locale !== 'en') {
                let fallback: any = builtinLocales['en'];
                for (const k of keys) {
                    if (fallback && typeof fallback === 'object' && k in fallback) {
                        fallback = fallback[k];
                    } else {
                        fallback = null;
                        break;
                    }
                }
                value = fallback;
            }

            if (typeof value === 'string') {
                // Determine replacements
                Object.keys(vars).forEach(k => {
                    const regex = new RegExp(`{{${k}}}`, 'g');
                    value = value.replace(regex, vars[k]);
                });
                return value;
            }

            return key; // Return key if not found
        };
    }
);
