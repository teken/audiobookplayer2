import { secondsToFormatted } from "./util";

export interface Book {
    id: string,
    name: string,
    author: string,
    series: string,

    files: string[],
    audio_files: string[],
    image_files: string[],
}

interface SubWork {
}

interface Work {
    subworks: SubWork[];
}

interface Author {
    works: Work[];
}

export interface Library {
    authors: Author[];
}

export class PlayerState {
    playing: boolean = false;
    muted: boolean = false;

    volumn: number = 1.0;
    volumnMax: number = 1.0;

    duration: number = 30749;
    position: number = 12045;

    chapters: Chapter[] = [{
        startTime: 0,
        endTime: 10000
    }, {
        startTime: 10000,
        endTime: 20000
    }, {
        startTime: 20000,
        endTime: 30749
    }];

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
        return this.chapters.map(x => ({
            startPosition: (x.startTime / this.duration) * 100,
            endPosition: (x.endTime / this.duration) * 100
        } as Segment));
    }
}

export interface Stats {
    booksNotInSeries: number;
    books: number;
    series: number;
    authors: number;
}

export class Chapter {
    startTime: number = 0;
    endTime: number = 1;
    title?: string = "test"
}

export class Segment {
    startPosition: number = 0;
    endPosition: number = 1;
}