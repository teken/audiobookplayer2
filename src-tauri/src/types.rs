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
