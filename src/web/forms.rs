use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::db::models::{BoardGame, BoardGameExpansion};
use crate::prelude::GENResult;
use crate::{utils, SETTINGS};

#[derive(Deserialize)]
pub struct AdminAuthForm {
    pub token: String,
}

#[derive(MultipartForm)]
#[multipart(deny_unknown_fields)]
#[multipart(duplicate_field = "deny")]
pub struct BGMultiPartForm {
    pub title: Text<String>,
    pub image: Option<TempFile>,
    pub min_players: Text<u8>,
    pub max_players: Text<u8>,
    pub players_no_limit: Option<Text<String>>,
    pub min_playtime: Text<u16>,
    pub max_playtime: Text<u16>,
    pub playtime_no_limit: Option<Text<String>>,
    pub available: Option<Text<String>>,
    pub meta_add_expansion: Option<Text<String>>,
    #[multipart(rename = "expansions[][title]")]
    pub expansion_titles: Vec<Text<String>>,
    #[multipart(rename = "expansions[][meta-del-expansion]")]
    pub meta_del_expansion: Vec<Text<u8>>,
}

impl BGMultiPartForm {
    pub fn save_image(&self, uid: &str) -> GENResult<Option<String>> {
        match &self.image {
            None => Ok(None),
            Some(image) => {
                let (filename, is_webp) = utils::verify_file_as_webp(image)?;
                if filename == "" {
                    return Ok(None);
                };
                let mut src = image.file.path().to_path_buf();
                let mut dest =
                    PathBuf::from(format!("{}/assets/{}", &SETTINGS.data_dir, uid)).join(filename);
                dest.set_extension("webp");

                if dest.parent().is_some() {
                    fs::create_dir_all(&dest.parent().unwrap())?;
                }
                if !is_webp {
                    src = utils::convert_img_to_webp(image.file.path())?;
                }
                utils::copy_file(&src, &dest)?;
                utils::delete_file(&src)?;

                let mut html_ref = PathBuf::from(format!("assets/{}", uid)).join(filename);
                html_ref.set_extension("webp");
                Ok(Some(html_ref.as_path().display().to_string()))
            }
        }
    }
}

#[derive(Serialize)]
pub struct BGForm {
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub min_players: Option<u8>,
    pub max_players: Option<u8>,
    pub players_no_limit: Option<bool>,
    pub min_playtime: Option<u16>,
    pub max_playtime: Option<u16>,
    pub playtime_no_limit: Option<bool>,
    pub available: Option<bool>,
    pub expansions: Vec<BGExpansionForm>,
}

#[derive(Serialize)]
pub struct BGExpansionForm {
    pub title: Option<String>,
}

impl From<BGMultiPartForm> for BGForm {
    fn from(form: BGMultiPartForm) -> Self {
        let mut exp: Vec<BGExpansionForm> = Vec::new();
        for (_i, e) in form.expansion_titles.iter().enumerate() {
            let title = e.0.clone(); //TODO better error handling
            exp.push(BGExpansionForm { title: Some(title) })
        }

        let playtime_no_limit = match form
            .playtime_no_limit
            .unwrap_or(Text("off".to_string()))
            .into_inner()
            .as_str()
        {
            "on" => true,
            _ => false,
        };

        let players_no_limit = match form
            .players_no_limit
            .unwrap_or(Text("off".to_string()))
            .into_inner()
            .as_str()
        {
            "on" => true,
            _ => false,
        };

        let available = match form
            .available
            .unwrap_or(Text("off".to_string()))
            .into_inner()
            .as_str()
        {
            "on" => true,
            _ => false,
        };

        BGForm {
            title: Some(form.title.0),
            image_url: Some("".to_string()),
            min_players: Some(form.min_players.0),
            max_players: Some(form.max_players.0),
            players_no_limit: Some(players_no_limit),
            min_playtime: Some(form.min_playtime.0),
            max_playtime: Some(form.max_playtime.0),
            playtime_no_limit: Some(playtime_no_limit),
            available: Some(available),
            expansions: exp,
        }
    }
}

impl From<BoardGame> for BGForm {
    fn from(bg: BoardGame) -> Self {
        let mut exps = Vec::new();
        for exp in bg.expansions {
            exps.push(BGExpansionForm {
                title: Some(exp.title),
            })
        }

        BGForm {
            title: Some(bg.title),
            image_url: Some(bg.image_url),
            min_players: Some(bg.min_players),
            max_players: Some(bg.max_players),
            players_no_limit: Some(bg.players_no_limit),
            min_playtime: Some(bg.min_playtime),
            max_playtime: Some(bg.max_playtime),
            playtime_no_limit: Some(bg.playtime_no_limit),
            available: Some(bg.available),
            expansions: exps,
        }
    }
}

impl From<BGMultiPartForm> for BoardGame {
    fn from(form: BGMultiPartForm) -> Self {
        let mut exp: Vec<BoardGameExpansion> = Vec::new();
        for (_i, e) in form.expansion_titles.iter().enumerate() {
            let title = e.0.clone(); //TODO better error handling
            exp.push(BoardGameExpansion { title })
        }

        let playtime_no_limit = match form
            .playtime_no_limit
            .unwrap_or(Text("off".to_string()))
            .into_inner()
            .as_str()
        {
            "on" => true,
            _ => false,
        };

        let players_no_limit = match form
            .players_no_limit
            .unwrap_or(Text("off".to_string()))
            .into_inner()
            .as_str()
        {
            "on" => true,
            _ => false,
        };

        let available = match form
            .available
            .unwrap_or(Text("off".to_string()))
            .into_inner()
            .as_str()
        {
            "on" => true,
            _ => false,
        };

        BoardGame {
            uid: None,
            title: form.title.0,
            image_url: "assets/placeholder.webp".to_string(),
            min_players: form.min_players.0,
            max_players: form.max_players.0,
            players_no_limit,
            min_playtime: form.min_playtime.0,
            max_playtime: form.max_playtime.0,
            playtime_no_limit,
            available,
            expansions: exp,
            bgg_id: None,
        }
    }
}
