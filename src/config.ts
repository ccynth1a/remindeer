import { writable } from "svelte/store";

export const theme = writable<string>('light');

export const toggleTheme = () => {
    theme.update(t => t == 'light' ? 'dark' : 'light');
}