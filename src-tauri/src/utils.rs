use lofty::AudioFile;
use lofty::TaggedFileExt;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;
use surrealdb::sql::Object;
use surrealdb::sql::Value;
use surrealdb::Response;

use crate::DB;
use crate::SES;

use crate::types::{Chapter, ReadFileMetadataError, TrackMetadata, Work};

pub const AUDIO_FILE_EXTENSIONS: [&str; 5] = ["mp4", "mp3", "m4a", "m4b", "wav"];
pub const AUDIO_FILE_WITH_CHAPTERS_EXTENSIONS: [&str; 3] = ["mp4", "m4a", "m4b"];
pub const IMAGE_FILE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

pub fn object_into_work(object: Object) -> Work {
    let series_str = object.get("series").map(|x| x.clone().as_string()).unwrap();
    let series = if series_str.is_empty() {
        None
    } else {
        Some(series_str)
    };

    let files: Vec<String> = if let Value::Array(sub_object) = object.get("files").unwrap() {
        sub_object
            .clone()
            .into_iter()
            .map(|x| x.as_string())
            .collect()
    } else {
        vec![]
    };
    Work {
        id: object.get("id").map(|x| x.clone().as_string()).unwrap(),
        name: object.get("name").map(|x| x.clone().as_string()).unwrap(),
        author: if let Value::Object(sub_object) = object.get("author").unwrap() {
            sub_object
                .get("name")
                .map(|x| x.clone().as_string())
                .unwrap()
        } else {
            "".to_string()
        },
        series,
        path: object.get("path").map(|x| x.clone().as_string()).unwrap(),
        files: files.clone(),
        audio_files: get_files_by_extension(files.clone(), AUDIO_FILE_EXTENSIONS.to_vec()),
        image_files: get_files_by_extension(files, IMAGE_FILE_EXTENSIONS.to_vec()),
    }
}

pub fn into_iter_objects(
    result: Vec<Response>,
) -> Result<impl Iterator<Item = Result<Object, String>>, String> {
    let resp = result.into_iter().next().map(|rp| rp.result).transpose()?;
    match resp {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err("record was not an object".to_owned()),
            });
            Ok(it)
        }
        _ => Err("Failed to find array in result".to_owned()),
    }
}

pub fn get_files_by_extension(files: Vec<String>, extenions: Vec<&str>) -> Vec<String> {
    files
        .iter()
        .filter(|x| {
            let path = PathBuf::from(x);
            path.extension().is_some()
                && extenions.contains(&path.extension().unwrap().to_str().unwrap())
        })
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}

pub fn string_to_id(item: String) -> String {
    Regex::new(r"[^a-zA-Z0-9]")
        .unwrap()
        .replace_all(item.as_str(), "_")
        .to_string()
}

pub async fn create_work(work: Work) -> Result<(), String> {
    let ass = format!("CREATE works CONTENT {{ name: $name, author: authors:{}, series: $series, path: $path, files:$files, }}", work.author);
    let data: BTreeMap<String, Value> = BTreeMap::from([
        ("name".into(), work.name.into()),
        (
            "series".into(),
            match work.series {
                Some(v) => v,
                None => "".to_owned(),
            }
            .into(),
        ),
        ("path".into(), work.path.into()),
        (
            "files".into(),
            work.files
                .iter()
                .map(String::as_str)
                .collect::<Vec<&str>>()
                .into(),
        ),
    ]);
    match DB
        .get()
        .expect("DB does not exist")
        .execute(ass.as_str(), &SES, Some(data), false)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("failed to create work: {}", err)),
    }
}

pub async fn create_author(author_name: String) -> Result<String, String> {
    let author_id = string_to_id(author_name.clone());
    let ass = format!(
        "CREATE authors:{} CONTENT {{ name: $name }}",
        author_id.clone()
    );
    let data: BTreeMap<String, Value> =
        BTreeMap::from([("name".into(), author_name.replace('\'', r"\'").into())]);
    match DB
        .get()
        .expect("DB does not exist")
        .execute(ass.as_str(), &SES, Some(data), false)
        .await
    {
        Ok(_) => Ok(author_id),
        Err(err) => Err(format!("failed to create work: {}", err)),
    }
}

pub fn read_file_metadata(path: String) -> Result<TrackMetadata, ReadFileMetadataError> {
    let Ok(file) = lofty::read_from_path(path.clone()) else {
        return Err(ReadFileMetadataError {});
    };

    let Some(tag) = file.primary_tag() else {
        return Err(ReadFileMetadataError {});
    };

    let mut chapters = vec![];

    let c_path = PathBuf::from(path.clone());
    if c_path.extension().is_some()
        && AUDIO_FILE_WITH_CHAPTERS_EXTENSIONS
            .contains(&c_path.extension().unwrap().to_str().unwrap())
    {
        let Ok(f) = File::open(path.clone())  else {
            return Err(ReadFileMetadataError {});
        };
        let size = f.metadata().unwrap().len();
        let reader = BufReader::new(f);
        let Ok(mut mp4) = mp4::Mp4Reader::read_header(reader, size)  else {
            return Err(ReadFileMetadataError {});
        };

        let Some(track) = mp4
            .tracks()
            .values()
            .find(|x| x.media_type().is_err()) else {
                return Err(ReadFileMetadataError {});
            };
        let track_id = track.track_id();
        for i in 0..track.sample_count() {
            let Ok(Some(sample)) = mp4.read_sample(track_id, i + 1) else {
                return Err(ReadFileMetadataError {});
            };

            chapters.push(Chapter {
                title: format!("Chapter {}", i),
                length: if sample.duration == 0 {
                    file.properties().duration() - Duration::from_millis(sample.start_time)
                } else {
                    Duration::from_millis(sample.duration.into())
                },
            });
        }
    } else {
        chapters.push(Chapter {
            title: tag
                .get_string(&lofty::ItemKey::TrackTitle)
                .unwrap_or_default()
                .to_owned(),
            length: file.properties().duration(),
        });
    }

    let metadata = TrackMetadata {
        path,
        duration: file.properties().duration(),
        track_title: tag
            .get_string(&lofty::ItemKey::TrackTitle)
            .unwrap_or_default()
            .to_owned(),
        track_author: tag
            .get_string(&lofty::ItemKey::TrackArtist)
            .unwrap_or_default()
            .to_owned(),
        album_title: tag
            .get_string(&lofty::ItemKey::AlbumTitle)
            .unwrap_or_default()
            .to_owned(),
        chapters,
    };

    Ok(metadata)
}
