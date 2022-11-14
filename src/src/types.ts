export interface Book {
    name: string,
    author: string,
    series: string,
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