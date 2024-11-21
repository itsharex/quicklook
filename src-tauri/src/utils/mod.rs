use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    file_type: String,
    path: String,
    extension: String,
}

impl File {
    // 构造 File 实例
    fn new(file_type: &str, path: String, extension: String) -> File {
        File {
            file_type: file_type.to_string(),
            path,
            extension,
        }
    }

    // 获取文件类型
    pub fn get_file_type(&self) -> String {
        self.file_type.clone()
    }

    // 获取文件路径
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    // 获取文件扩展名
    pub fn get_extension(&self) -> String {
        self.extension.clone()
    }
}

pub fn get_file_info(path: &str) -> Option<File> {
    let file_path = Path::new(path);
    let path_str = path.to_string();

    // 获取文件扩展名，如果没有扩展名，默认使用 "txt"
    let extension = file_path
        .extension()
        .map_or("txt".to_string(), |ext| ext.to_string_lossy().into_owned());

    // 根据扩展名从映射表中获取文件类型
    match file_type_mapping().get(extension.as_str()) {
        Some(file_type) => Some(File::new(file_type, path_str, extension)),
        None => None, // 如果没有匹配的文件类型，返回 None
    }
}

// 返回一个文件扩展名到文件类型的映射
fn file_type_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // 文档文件
    map.insert("markdown", "Markdown");
    map.insert("md", "Markdown");
    // DOC 文件
    map.insert("doc", "Doc");
    map.insert("docx", "Doc");
    map.insert("xls", "Doc");
    map.insert("xlsx", "Doc");
    map.insert("ppt", "Doc");
    map.insert("pptx", "Doc");

    // 字体文件
    map.insert("ttf", "Font");
    map.insert("otf", "Font");
    map.insert("woff2", "Font");
    map.insert("woff", "Font");

    // 图片文件
    map.insert("jpg", "Image");
    map.insert("jpeg", "Image");
    map.insert("png", "Image");
    map.insert("gif", "Image");
    map.insert("webp", "Image");
    map.insert("bmp", "Image");
    map.insert("ico", "Image");
    map.insert("svg", "Image");
    map.insert("apng", "Image");

    // 视频文件
    map.insert("mp4", "Video");
    map.insert("webm", "Video");
    map.insert("mkv", "Video");
    map.insert("flv", "Video");
    map.insert("avi", "Video");
    map.insert("mov", "Video");
    map.insert("wmv", "Video");
    map.insert("mpg", "Video");
    map.insert("mpeg", "Video");
    map.insert("m4v", "Video");
    map.insert("3gp", "Video");
    map.insert("3g2", "Video");

    // 音频文件
    map.insert("mp3", "Audio");
    map.insert("wav", "Audio");
    map.insert("flac", "Audio");
    map.insert("ogg", "Audio");
    map.insert("m4a", "Audio");
    map.insert("wma", "Audio");
    map.insert("aac", "Audio");
    map.insert("amr", "Audio");
    map.insert("aiff", "Audio");
    map.insert("au", "Audio");
    map.insert("awb", "Audio");
    map.insert("dct", "Audio");
    map.insert("dss", "Audio");
    map.insert("dvf", "Audio");
    map.insert("gsm", "Audio");
    map.insert("iklax", "Audio");
    map.insert("ivs", "Audio");
    map.insert("m4p", "Audio");
    map.insert("mmf", "Audio");
    map.insert("mpc", "Audio");
    map.insert("msv", "Audio");
    map.insert("nmf", "Audio");
    map.insert("nsf", "Audio");
    map.insert("ra", "Audio");
    map.insert("rm", "Audio");
    map.insert("sln", "Audio");
    map.insert("tta", "Audio");
    map.insert("vox", "Audio");
    map.insert("wv", "Audio");
    map.insert("8svx", "Audio");
    map.insert("cda", "Audio");
    map.insert("mid", "Audio");
    map.insert("midi", "Audio");
    map.insert("mka", "Audio");

    // 压缩文件
    map.insert("zip", "Archive");
    map.insert("rar", "Archive");
    map.insert("7z", "Archive");

    // 应用程序文件
    map.insert("exe", "App");
    map.insert("dmg", "App");
    map.insert("deb", "App");
    map.insert("rpm", "App");
    map.insert("apk", "App");
    map.insert("appimage", "App");

    // 代码文件
    map.insert("cpp", "Code");
    map.insert("js", "Code");
    map.insert("mjs", "Code");
    map.insert("ts", "Code");
    map.insert("mts", "Code");
    map.insert("tsx", "Code");
    map.insert("rs", "Code");
    map.insert("py", "Code");
    map.insert("java", "Code");
    map.insert("html", "Code");
    map.insert("css", "Code");
    map.insert("scss", "Code");
    map.insert("sass", "Code");
    map.insert("less", "Code");
    map.insert("c", "Code");
    map.insert("go", "Code");
    map.insert("vue", "Code");
    map.insert("jsx", "Code");
    map.insert("json", "Code");
    map.insert("yml", "Code");
    map.insert("yaml", "Code");
    map.insert("toml", "Code");
    map.insert("bat", "Code");
    map.insert("ps1", "Code");
    map.insert("ini", "Code");
    map.insert("swift", "Code");
    map.insert("kt", "Code");
    map.insert("php", "Code");
    map.insert("h", "Code");
    map.insert("xml", "Code");

    // 书籍文件
    map.insert("pdf", "Book");

    // 文本文件（默认）
    map.insert("txt", "Text");

    map
}
