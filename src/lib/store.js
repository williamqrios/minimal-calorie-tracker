import { writable, readable } from 'svelte/store';

export let foodsNormalized = writable([]);
export let recipes = writable([]);
export let dailyTotals = writable({});
export let logId = writable(null);
export let userGoal = writable({});
export const today = readable(new Date(), () => {});
