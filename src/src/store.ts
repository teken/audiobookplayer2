import { writable } from 'svelte/store';
import { secondsToFormatted } from './util';

export const search = writable("");
export const displaySearchResults = writable(false);

export class PlayerState {
    playing: boolean = false;
    muted: boolean = false;

    volumn: number = 1.0;
    volumnMax: number = 1.0;

    duration: number = 30749;
    position: number = 12045;

    get positionAsPercentage() {
        return (this.position / this.duration) * 100;
    }

    get volumnAsPercentage() {
        return (this.volumn / this.volumnMax) * 100;
    }

    get positionFormatted() {
        return secondsToFormatted(this.position);
    }

    get durationFormatted() {
        return secondsToFormatted(this.duration);
    }
}

export const playerState = writable<PlayerState>(new PlayerState())