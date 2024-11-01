# quicklook

windows 平台的文件预览工具

## 如何运行项目

### 前置依赖

- Rust [官方网站](https://www.rust-lang.org/tools/install)
- Tauri [官方网站](https://tauri.app/start/prerequisites/)
- NodeJS [官方网站](https://nodejs.org/)

### 拉取项目代码

```bash
git clone https://github.com/GuoJikun/quicklook.git 
```

### 运行项目

> 推荐使用 pnpm

```bash
pnpm i #安装项目依赖
pnpm tauri dev 运行项目
```

### 打包

```bash
pnpm tauri build
```

## 支持预览文件

> 未完全完成

- Image: svg\png\apng\jpg\jpeg\gif\bmp\webp
- Markdown: md\markdown
- Doc: xlsx\xls\csv\ods\tsv\
- Syntx Highlight: txt\js\c++\h++\
- Font: ttf\otf\woff\woff2
