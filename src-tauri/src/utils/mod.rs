use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    file_type: String,
    path: String,
    extension: String,
}

// impl File {
//     pub fn file_type(&self) -> String {
//         return self.file_type.clone();
//     }
//     pub fn path(&self) -> String {
//         return self.path.clone();
//     }
//     pub fn extension(&self) -> String {
//         return self.extension.clone();
//     }

// }

pub fn get_file_info(path: &String) -> Option<File> {
    let file_path = Path::new(path);
    let path_str = path.to_string();
    match file_path.extension() {
        Some(extension) => {
            let extension = extension.to_string_lossy().into_owned();
            match extension.as_str() {
                "markdown" | "md" => Some(File {
                    file_type: "Markdown".to_string(),
                    path: path_str,
                    extension,
                }),
                "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => Some(File {
                    file_type: "Doc".to_string(),
                    path: path_str,
                    extension,
                }),
                "ttf" | "otf" | "woff2" | "woff" => Some(File {
                    file_type: "Font".to_string(),
                    path: path_str,
                    extension,
                }),
                "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "ico" | "svg" | "apng" => {
                    Some(File {
                        file_type: "Image".to_string(),
                        path: path_str,
                        extension,
                    })
                }
                "mp4" | "webm" | "mkv" | "flv" | "avi" | "mov" | "wmv" | "mpg" | "mpeg" | "m4v"
                | "3gp" | "3g2" => Some(File {
                    file_type: "Video".to_string(),
                    path: path_str,
                    extension,
                }),
                "mp3" | "wav" | "flac" | "ogg" | "m4a" | "wma" | "aac" | "amr" | "aiff" | "au"
                | "awb" | "dct" | "dss" | "dvf" | "gsm" | "iklax" | "ivs" | "m4p" | "mmf"
                | "mpc" | "msv" | "nmf" | "nsf" | "ra" | "rm" | "sln" | "tta" | "vox" | "wv"
                | "8svx" | "cda" | "mid" | "midi" | "mka" => Some(File {
                    file_type: "Audio".to_string(),
                    path: path_str,
                    extension,
                }),
                "zip" | "rar" | "7z" => Some(File {
                    file_type: "Archive".to_string(),
                    path: path_str,
                    extension,
                }),
                "exe" | "dmg" | "deb" | "rpm" | "apk" | "appimage" => Some(File {
                    file_type: "App".to_string(),
                    path: path_str,
                    extension,
                }),
                "cpp" | "js" | "mjs" | "ts" | "mts" | "tsx" | "rs" | "py" | "java" | "html" | "css"
                | "scss" | "sass" | "less" | "c" | "go" | "vue" | "jsx" | "json" | "yml"
                | "yaml" | "toml" | "bat" | "ps1" | "ini" | "swift" | "kt" | "php" | "h" | "xml" => Some(File {
                    file_type: "Code".to_string(),
                    path: path_str,
                    extension,
                }),
                "pdf" => Some(File {
                    file_type: "Book".to_string(),
                    path: path_str,
                    extension,
                }),
                "txt" => Some(File {
                    file_type: "Text".to_string(),
                    path: path_str,
                    extension,
                }),
                _ => None,
            }
        }
        None => Some(File {
            file_type: "Text".to_string(),
            path: path_str,
            extension: String::from("txt"),
        }),
    }
}
