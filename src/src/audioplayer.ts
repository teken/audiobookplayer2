import { emit } from "@tauri-apps/api/event";
import { tauri } from "@tauri-apps/api";
import { secondsToFormatted } from "./util";
import type { Segment } from "./types";

export class AudioPlayer {
    audio: HTMLAudioElement;
    files: string[] = [];
    playingFileIndex: number = 0;
    skipMode: 'files' | 'chapters' = 'files';

    constructor() {
        this.audio = new Audio();

        this.audio.addEventListener("timeupdate", () => {
            emit("update_file_position", {
                position: this.audio.currentTime,
                duration: this.audio.duration,
                fileIndex: this.playingFileIndex,
            });
        });

        this.audio.addEventListener("ended", () => {
            console.log("ended");
            if (this.playingFileIndex + 1 < this.files.length - 1) {
                this.loadfile(this.files[++this.playingFileIndex]);
                this.play();
            }
        });
    }

    play() {
        this.audio.play();
    }
    pause() {
        this.audio.pause();
    }
    unload() {
        this.files = [];
        this.audio.src = "";
        this.audio.load();
    }
    load(files: string[]) {
        if (files.length == 1) this.skipMode = 'chapters';
        this.playingFileIndex = 0;
        this.files = files;
        this.loadfile(files[this.playingFileIndex]);

    }
    loadfile(file: string, position: number = 0, autoPlay: boolean = false) {
        this.audio.src = tauri.convertFileSrc(file);
        this.audio.load();
        this.audio.currentTime = position;
        if (autoPlay) this.play();
    }
    foward() {
        if (this.skipMode == "files" && this.playingFileIndex + 1 < this.files.length - 1)
            this.loadfile(this.files[++this.playingFileIndex]);
    }
    backward() {
        if (this.skipMode == "files" && this.playingFileIndex > 0)
            this.loadfile(this.files[--this.playingFileIndex]);
    }
    setPosition(payload: {
        position: number;
        index: number;
    }) {
        if (payload.index != this.playingFileIndex) {
            this.playingFileIndex = payload.index;
            this.loadfile(this.files[payload.index], payload.position, true);
        } else
            this.audio.currentTime = payload.position;
    }

    setVolumn(volumn: number) {
        this.audio.volume = volumn;
    }
}

export class PlayerState {
    ready: boolean = false;
    playing: boolean = false;
    muted: boolean = false;

    volumn: number = 1.0;
    volumnMax: number = 1.0;

    filePosition: number = 0;
    fileIndexPosition: number = 0;
    fileMetadata: TrackMetadata[] = [];

    get chapters(): Chapter[] {
        return this.fileMetadata.reduce((a, v) => a.concat(v.chapters ?? []), []);
    }

    get duration() {
        return this.fileMetadata.reduce((acc, x) => acc + x.duration.secs, 0);
    }

    get position() {
        return this.fileMetadata.reduce((acc, x, i) => {
            if (i < this.fileIndexPosition) return acc + x.duration.secs;
            if (i == this.fileIndexPosition) return acc + this.filePosition;
            return acc;
        }, 0);
    }

    updatePosition(value: number) {
        this.position = value;

        emit("set_file_position", {
            position: this.filePosition,
            index: this.fileIndexPosition,
        });
    }

    set position(value: number) {
        let sum = 0;

        for (const x of this.fileMetadata) {
            if (sum + x.duration.secs > value) {
                this.filePosition = value - sum;
                this.fileIndexPosition = this.fileMetadata.indexOf(x);
                break;
            }
            sum += x.duration.secs;
        }
    }

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

    get chaptersAsSegments() {
        let sum = 0;
        return this.chapters.map(x => {
            const seg = {
                startPosition: (sum / this.duration) * 100,
                endPosition: ((sum + x.length.secs) / this.duration) * 100
            } as Segment
            sum += x.length.secs;
            return seg;
        });
    }

    constructor() { }
}

export class Chapter {
    length: Duration = { secs: 1, nanos: 1 };
    title?: string = "test"
}

export interface TrackMetadata {
    path: string;
    trackTitle: string;
    tractAuthor: string;
    albumTitle: string;
    duration: Duration;
    chapters: Chapter[];
}

export interface Duration {
    secs: number;
    nanos: number;
}