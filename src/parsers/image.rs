use serde::{Deserialize, Serialize};
use uuid::Uuid; 
use chrono::{Utc, DateTime};
use std::fs::{File};
use exif::{Reader, Exif, Tag, In, Value};
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: f64, // lon
    pub y: f64, // lat
    pub srid: Option<i32> // spatial reference identifier
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageMetaData {
    exif_version: Option<f64>,
    x_pixel_dimension: Option<u32>,
    y_pixel_dimension: Option<u32>,
    x_resolution: Option<u32>,
    y_resolution: Option<u32>,
    date_of_image: Option<DateTime<Utc>>,
    flash: Option<bool>,
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    f_number: Option<String>,
    aperture_value: Option<f64>,
    gps_point: Option<Point>,
    altitude: Option<f64>,
    speed: Option<f64>,
    media_item_id: Uuid,
}

impl ImageMetaData {
    pub fn empty(id: Uuid) -> ImageMetaData {
        ImageMetaData {
            exif_version: None,
            x_pixel_dimension: None,
            y_pixel_dimension: None,
            x_resolution: None,
            y_resolution: None,
            date_of_image: None,
            flash: None,
            make: None,
            model: None,
            exposure_time: None,
            f_number: None,
            aperture_value: None,
            gps_point: None,
            altitude: None,
            speed: None,
            media_item_id: id
        }
    }
}

pub fn parse(media_id: Uuid, file_path: &str) -> Result<ImageMetaData, Box<dyn std::error::Error>> {
    let file = File::open(file_path).expect("Cannot open the file.");
    
    let result = 
        Reader::new()
        .read_from_container(&mut BufReader::new(file));
    
    match result {
        Ok(reader) => {
            Ok(
                ImageMetaData {
                    exif_version: get_float(&reader, Tag::ExifVersion),
                    x_pixel_dimension: get_int(&reader, Tag::PixelXDimension),
                    y_pixel_dimension: get_int(&reader, Tag::PixelYDimension),
                    x_resolution: get_int(&reader, Tag::XResolution),
                    y_resolution: get_int(&reader, Tag::YResolution),
                    date_of_image: get_datetime(&reader, Tag::DateTime),
                    flash: get_flash(&reader),
                    make: get_string(&reader, Tag::Make),
                    model: get_string(&reader, Tag::Model),
                    exposure_time: get_string(&reader, Tag::ExposureTime),
                    f_number: get_string(&reader, Tag::FNumber),
                    aperture_value: get_float(&reader, Tag::ApertureValue),
                    gps_point: get_geo(&reader),
                    altitude: get_float(&reader, Tag::GPSAltitude),
                    speed: get_float(&reader, Tag::GPSSpeed),
                    media_item_id: media_id
                }
            )  
        }
        Err(e) => {
            Err(Box::new(e))
        }
    }    
}

fn calculate_point(reader: &Exif, dms: Tag, dms_ref: Tag) -> f64 {
    // get latitude
    match reader.get_field(dms, In::PRIMARY) {
        Some(field) => {
            match field.value {
                Value::Rational(ref vec) if !vec.is_empty() => { 
                    let deg = vec[0].to_f64();
                    let min = vec[1].to_f64();
                    let sec = vec[2].to_f64();
                    let ref_factor = calculate_ref(&reader, dms_ref);
                    convert_point(deg, min, sec) * ref_factor
                },
                _ => 0.0
            }
        },
        None => 0.0
    }
}

fn calculate_ref(reader: &Exif, dms_ref: Tag) -> f64 {
    match get_string(&reader, dms_ref) {
        Some(field) => {
            match field.as_ref() {
                "N" => 1.0,
                "S" => -1.0,
                "E" => 1.0,
                "W" => -1.0,
                _ => 1.0
            }
        },
        None => 1.0
    }
}

fn get_float(reader: &Exif, tag: Tag) -> Option<f64> {
    reader.get_field(tag, In::PRIMARY) 
        .and_then(|filed| match filed.value {
           Value::Rational(ref vec) if !vec.is_empty() => Some(vec[0].to_f64()),
           _ => None
        })
}

fn get_int(reader: &Exif, tag: Tag) -> Option<u32> {
    reader.get_field(tag, In::PRIMARY)
        .and_then(|filed| filed.value.get_uint(0))
}

fn get_string(reader: &Exif, tag: Tag) -> Option<String> {
    reader.get_field(tag, In::PRIMARY)
        .and_then(|field| Some(field.value.display_as(tag).to_string()))
}

fn get_datetime(reader: &Exif, tag: Tag) -> Option<DateTime<Utc>> {
    use chrono::offset::TimeZone;
    match reader.get_field(tag, In::PRIMARY) {
        Some(field) => {
            let val = field.value.display_as(tag).to_string();
            Utc.datetime_from_str(&val, "%Y-%m-%d %H:%M:%S").ok()
        },
        None => None
    }
}

fn get_flash(reader: &Exif) -> Option<bool> {
    match get_string(&reader, Tag::Flash) {
        Some(flash) => {
            Some(flash.starts_with("fired"))
    },
        None => None
    }
}

fn get_geo(reader: &Exif) -> Option<Point> {
    let latitude = calculate_point(&reader, Tag::GPSLatitude, Tag::GPSLatitudeRef);
    let longitude = calculate_point(&reader, Tag::GPSLongitude, Tag::GPSLongitudeRef);
    
    if latitude == 0.00 || longitude == 0.0 {
        None
    } else {
        Some(Point {
            x: longitude,
            y: latitude,
            srid: None
        })
    }
}

fn convert_point(deg: f64, min: f64, sec: f64) -> f64 {
    deg + (min / 60.0 ) + (sec / 3600.0 ) 
}