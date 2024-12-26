use std::{ fs::File, io::BufReader};

use serde_json::Value;
use tauri::{AppHandle, Manager, path::BaseDirectory};

// 读取resources下的config.json文件
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub markdown: Vec<String>,
    pub markdown_checked: Vec<String>,
    pub image: Vec<String>,
    pub image_checked: Vec<String>,
    pub video: Vec<String>,
    pub video_checked: Vec<String>,
    pub doc: Vec<String>,
    pub doc_checked: Vec<String>,
    pub code: Vec<String>,
    pub code_checked: Vec<String>,
    pub font: Vec<String>,
    pub font_checked: Vec<String>,
    pub archive: Vec<String>,
    pub archive_checked: Vec<String>,
    pub book: Vec<String>,
    pub book_checked: Vec<String>,
}

pub fn read_config(app: &AppHandle) -> Result<Config, String> {
    let config_path = app.path().resolve("config.json", BaseDirectory::Resource).unwrap();

    let file = File::open(config_path).map_err(|e|e.to_string())?;
    let reader = BufReader::new(file);
    let config: Value = serde_json::from_reader(reader).map_err(|e|e.to_string())?;
    let config = config.as_object().ok_or("config.json is not an object")?;
    Ok(Config {
        markdown: config.get("preview.markdown").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        markdown_checked: config.get("preview.markdown.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        image: config.get("preview.image").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        image_checked: config.get("preview.image.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        video: config.get("preview.video").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        video_checked: config.get("preview.video.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        doc: config.get("preview.doc").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        doc_checked: config.get("preview.doc.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        code: config.get("preview.code").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        code_checked: config.get("preview.code.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        font: config.get("preview.font").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        font_checked: config.get("preview.font.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        archive: config.get("preview.archive").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        archive_checked: config.get("preview.archive.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        book: config.get("preview.book").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
        book_checked: config.get("preview.book.checked").unwrap().as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_string()).collect(),
    })
}

