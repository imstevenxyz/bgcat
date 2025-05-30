use std::{
    fs,
    path::{Path, PathBuf},
};

use actix_multipart::form::tempfile::TempFile;
use log::{info, warn};
use mime::Mime;

use crate::SETTINGS;
use crate::{errors::BGCError, prelude::GENResult};

pub fn setup_data_dir() -> GENResult<()> {
    let assets_dir = format!("{}/assets", &SETTINGS.data_dir);
    fs::create_dir_all(assets_dir)?;
    Ok(())
}

pub fn verify_file_as_webp(file: &TempFile) -> GENResult<(&str, bool)> {
    let filename = get_file_name(file)?;
    if filename == "" {
        //Sometimes an empty file is detected even when no image is selected or uploaded
        return Ok(("", false));
    }
    let mime = get_file_mime(file)?;
    verify_file_size(file)?;
    verify_img_mimetype(&mime)?;
    Ok((filename, mime_is_webp(mime)))
}

fn get_file_name(file: &TempFile) -> GENResult<&str> {
    match &file.file_name {
        Some(name) => Ok(name),
        None => Err(BGCError::UserError("Bad image file name".to_string())),
    }
}

fn get_file_mime(file: &TempFile) -> GENResult<&Mime> {
    match file.content_type.as_ref() {
        Some(mime) => Ok(mime),
        None => Err(BGCError::UserError("Bad image mime type".to_string())),
    }
}

fn verify_file_size(file: &TempFile) -> GENResult<()> {
    if file.size <= 0 {
        return Err(BGCError::UserError("Bad image file size".to_string()));
    }
    Ok(())
}

fn verify_img_mimetype(mime: &Mime) -> GENResult<()> {
    match mime.type_() {
        mime::IMAGE => Ok(()),
        _ => Err(BGCError::UserError("Bad image mime type".to_string())),
    }
}

fn mime_is_webp(mime: &Mime) -> bool {
    match mime.subtype().as_str() {
        "webp" => true,
        _ => false,
    }
}

pub fn copy_file<F: AsRef<Path>, T: AsRef<Path>>(from: &F, to: &T) -> GENResult<()> {
    fs::copy(from, to)?;
    Ok(())
}

pub fn delete_file<F: AsRef<Path>>(path: &F) -> GENResult<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn delete_directory<F: AsRef<Path>>(path: &F) -> GENResult<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

pub fn delete_assets(uid: &str) {
    let result = delete_directory(&format!("{}/assets/{}", &SETTINGS.data_dir, uid));
    match result {
        Ok(_) => info!("Deleted assets of {}", uid),
        Err(why) => warn!("Failed to delete assets directory for {}: {}", uid, why),
    }
}

pub fn convert_img_to_webp(file: &Path) -> GENResult<PathBuf> {
    let image = image::ImageReader::open(file)?
        .with_guessed_format()?
        .decode()?;
    let webp = webp::Encoder::from_image(&image)?.encode(SETTINGS.webp_quality);
    let mut dest = PathBuf::from(file);
    dest.set_extension("webp");
    fs::write(&dest.as_path(), &*webp)?;
    Ok(dest)
}
