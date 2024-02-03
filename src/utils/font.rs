use font_kit::font::Font;
use log::error;
use rust_embed::RustEmbed;
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::sync::Arc;
use std::{env, fs, io, os};

#[derive(RustEmbed)]
#[folder = "resources/"]
struct FontAssets;

pub fn get_font(font_name: &str) -> Result<Option<Font>, Box<dyn Error>> {
    match FontAssets::get(font_name) {
        Some(assets) => {
            let font = Font::from_bytes(Arc::new(Vec::from(assets.data)), 0)?;
            Ok(Some(font))
        }
        None => {
            error!("Unable to find the specified font.");
            Ok(None)
        }
    }
}
