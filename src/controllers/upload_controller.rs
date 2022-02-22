use std::fs::FileType;

use uuid::Uuid;
use actix_web::{App, get};
use crate::parsers::image::parse as image_parser;
use crate::parsers::video::parse as video_parser;

#[get("/upload")]
pub async fn upload() -> String {
    String::from("hello")
}

fn parse_metadata(uuid: &Uuid, file_type: FileType, )
