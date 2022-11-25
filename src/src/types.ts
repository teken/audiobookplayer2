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