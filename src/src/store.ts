import { writable } from 'svelte/store';
import { PlayerState } from './audioplayer';

export const search = writable("");
export const displaySearchResults = writable(false);
export const playerState = writable<PlayerState>(new PlayerState())
