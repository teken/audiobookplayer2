import { writable } from 'svelte/store';
import { PlayerState } from './audioplayer';
import type { Settings } from './types';

export const search = writable("");
export const displaySearchResults = writable(false);
export const playerState = writable<PlayerState>(new PlayerState())
export const settings = writable<Settings>()
