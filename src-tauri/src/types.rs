use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackMetadata {
    pub path: String,
    pub track_title: String,
    pub track_author: String,
    pub album_title: String,
    pub duration: Duration,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub title: String,
    pub length: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadWorkDataError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddWorkTimeError;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Work {
    pub id: String,
    pub name: String,
    pub author: String,
    pub series: Option<String>,

    pub path: String,
    pub files: Vec<String>,
    pub image_files: Vec<String>,
    pub audio_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearDatabaseError;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadWorksError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadFileMetadataError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub library_location: String,
    pub library_style: LibraryStyle,
    // pub metadata_template: MetadataTemplate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LibraryStyle {
    Folder,
    Metadata,
}

pub struct MetadataTemplate {
    pub author: Vec<lofty::ItemKey>,
    pub title: Vec<lofty::ItemKey>,
}

impl Default for MetadataTemplate {
    fn default() -> Self {
        Self {
            author: vec![
                lofty::ItemKey::TrackArtist,
                lofty::ItemKey::OriginalArtist,
                lofty::ItemKey::AlbumArtist,
            ],
            title: vec![
                lofty::ItemKey::AlbumTitle,
                lofty::ItemKey::OriginalAlbumTitle,
                lofty::ItemKey::TrackTitle,
            ],
        }
    }
}
