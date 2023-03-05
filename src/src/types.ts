export interface Book {
    id: string,
    name: string,
    author: string,
    series: string,

    path: string,
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

export interface Stats {
    booksNotInSeries: number;
    books: number;
    series: number;
    authors: number;
}

export class Segment {
    startPosition: number = 0;
    endPosition: number = 1;
}