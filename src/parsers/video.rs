use serde::{Deserialize, Serialize};
use mp4parse::{MediaContext, Track, TryVec, TrackType, SampleEntry};
use uuid::Uuid;
use std::fs::File;


#[derive(Deserialize, Serialize, Debug)]
pub struct VideoMetaData {
    video_duration: Option<u64>,
    video_width: Option<u32>,
    video_height: Option<u32>,
    video_codec: Option<String>,
    audio_track_id: Option<u32>,
    audio_codec: Option<String>, 
    media_item_id: Uuid,
}

impl VideoMetaData {
    fn new(id: Uuid) -> VideoMetaData {
        VideoMetaData {
            video_duration: None,
            video_width: None,
            video_height: None,
            video_codec: None,
            audio_track_id: None,
            audio_codec: None,
            media_item_id: id,
        }
    }
}

pub fn parse(uuid: Uuid, file_path: &str) -> Result<VideoMetaData, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    match mp4parse::read_mp4(&mut file) { 
        Ok(mut context) => {
            // return the meta data
            Ok(create_meta_data(uuid, &mut context)) 
        },
        Err(_) => {
        // return a blank object
            Ok(VideoMetaData::new(uuid))
        }
    }
}

fn create_meta_data(uuid: Uuid, context: &mut MediaContext) -> VideoMetaData {
    let nvmd = VideoMetaData::new(uuid);
    checks_track(&mut context.tracks, nvmd)
}

fn checks_track(tracks: &mut TryVec<Track>, mut vdata: VideoMetaData) -> VideoMetaData {
    // copy from the book <rust for the iot>.
    for track in tracks.iter() {
        match track.track_type {
            TrackType::Video => {
                vdata.video_duration = Some(track.duration.unwrap().0);

                match &track.tkhd {
                    Some(tkhd) => {
                        vdata.video_height = Some(tkhd.height);
                        vdata.video_height = Some(tkhd.width);
                    }
                    None => ()
                }

                vdata.video_codec = match &track.stsd {
                    Some(sample) => {
                        match sample.descriptions.first().unwrap() {
                            SampleEntry::Video(v) => Some(format!("{:?}", v.codec_type)),
                            _ => None
                        }
                    }
                    None => None
                }
            }
            
            TrackType::Audio => {
                vdata.audio_track_id = Some(track.track_id.unwrap());
                vdata.audio_codec = match &track.stsd {
                    Some(sample) => {
                        match sample.descriptions.first().unwrap() {
                            SampleEntry::Audio(v) => Some(format!("{:?}", v.codec_type)),
                            _ => None
                        }
                    }
                    None => None
                }
            }
     
            TrackType::Metadata | TrackType::Unknown => {()}
        } 
    }
    
    vdata
}