use std::fs::File;

use serde::Serialize;
use zip::{DateTime, ZipArchive};

#[allow(unused)]
#[derive(Debug, Clone, Serialize)]
pub struct Extract {
    name: String,
    size: u64,
    last_modified: String,
    dir: bool,
}

#[allow(unused)]
impl Extract {
    pub fn zip(zip_path: &str) -> Result<Vec<Extract>, Box<dyn std::error::Error>> {
        // 打开压缩文件
        let file = File::open(zip_path)?;
        let mut archive: ZipArchive<File> = ZipArchive::new(file)?;
        let mut target: Vec<Extract> = vec![];
        // 遍历压缩包中的每一个文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let dir = file.is_dir();
            let file_name = file.name().to_string();
            let size = file.size();
            let last_modified = file
                .last_modified()
                .unwrap_or(DateTime::default())
                .to_string();

            target.push(Extract {
                name: file_name,
                size,
                last_modified,
                dir,
            });
        }

        Ok(target)
    }
}