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

export const PossibleTags = [
    "AlbumTitle",
    "SetSubtitle",
    "ShowName",
    "ContentGroup",
    "TrackTitle",
    "TrackSubtitle",

    // Original names
    "OriginalAlbumTitle",
    "OriginalArtist",
    "OriginalLyricist",

    // Sorting
    "AlbumTitleSortOrder",
    "AlbumArtistSortOrder",
    "TrackTitleSortOrder",
    "TrackArtistSortOrder",
    "ShowNameSortOrder",
    "ComposerSortOrder",

    // People & Organizations
    "AlbumArtist",
    "TrackArtist",
    "Arranger",
    "Writer",
    "Composer",
    "Conductor",
    "Director",
    "Engineer",
    "InvolvedPeople",
    "Lyricist",
    "MixDj",
    "MixEngineer",
    "MusicianCredits",
    "Performer",
    "Producer",
    "Publisher",
    "Label",
    "InternetRadioStationName",
    "InternetRadioStationOwner",
    "Remixer",

    // Counts & Indexes
    "DiscNumber",
    "DiscTotal",
    "TrackNumber",
    "TrackTotal",
    "Popularimeter",
    "ParentalAdvisory",

    // Dates
    "RecordingDate",
    "Year",
    "OriginalReleaseDate",

    // Identifiers
    "ISRC",
    "Barcode",
    "CatalogNumber",
    "Work",
    "Movement",
    "MovementNumber",
    "MovementTotal",

    // Flags
    "FlagCompilation",
    "FlagPodcast",

    // File Information
    "FileType",
    "FileOwner",
    "TaggingTime",
    "Length",
    "OriginalFileName",
    "OriginalMediaType",

    // Encoder information
    "EncodedBy",
    "EncoderSoftware",
    "EncoderSettings",
    "EncodingTime",
    "ReplayGainAlbumGain",
    "ReplayGainAlbumPeak",
    "ReplayGainTrackGain",
    "ReplayGainTrackPeak",

    // URLs
    "AudioFileURL",
    "AudioSourceURL",
    "CommercialInformationURL",
    "CopyrightURL",
    "TrackArtistURL",
    "RadioStationURL",
    "PaymentURL",
    "PublisherURL",

    // Style
    "Genre",
    "InitialKey",
    "Color",
    "Mood",
    "BPM",

    // Legal
    "CopyrightMessage",
    "License",

    // Podcast
    "PodcastDescription",
    "PodcastSeriesCategory",
    "PodcastURL",
    "PodcastReleaseDate",
    "PodcastGlobalUniqueID",
    "PodcastKeywords",

    // Miscellaneous
    "Comment",
    "Description",
    "Language",
    "Script",
    "Lyrics",

    // Vendor-specific
    "AppleXid",
    "AppleId3v2ContentGroup", // GRP1
]