use ffmpeg_next::{
    codec::context::Context as CodecContext,
    format::{input, Pixel},
    frame::Video as VideoFrame,
    media::Type,
    software::scaling::Context as ScalingContext,
    Error as FFmpegError,
};
use mp4::{AvcConfig, MediaConfig, Mp4Config, Mp4Sample, Mp4Writer, TrackConfig};
use std::io::{Cursor, Read};
use tauri::{AppHandle, Emitter, EventTarget};

pub fn decode_video_stream(app: &AppHandle, path: &str, label: String) -> Result<(), String> {
    ffmpeg_next::init().map_err(|e| e.to_string())?;

    // 打开视频文件
    let mut ictx = input(path).map_err(|e| e.to_string())?;

    let stream = ictx
        .streams()
        .best(Type::Video)
        .ok_or("No video stream found")?;

    // 获取视频流
    let stream_index = ictx
        .streams()
        .best(Type::Video)
        .ok_or("No video stream found")?
        .index();

    // 获取解码器
    let context_decoder =
        CodecContext::from_parameters(stream.parameters()).map_err(|e| e.to_string())?;
    let mut context = context_decoder
        .decoder()
        .video()
        .map_err(|e| e.to_string())?;

    // 创建MP4写入器
    let mut buffer = vec![0u8; 1024 * 1024];
    let writer = Cursor::new(&mut buffer);
    let config = Mp4Config {
        major_brand: str::parse("isom").unwrap(),
        minor_version: 512,
        compatible_brands: vec![
            str::parse("isom").unwrap(),
            str::parse("iso2").unwrap(),
            str::parse("avc1").unwrap(),
            str::parse("mp41").unwrap(),
        ],
        timescale: 1000,
    };
    let mut mp4_writer = Mp4Writer::write_start(writer, &config).map_err(|e| e.to_string())?;

    // 配置MP4轨道
    let track_config = TrackConfig {
        track_type: mp4::TrackType::Video,
        timescale: 1000,
        language: "und".to_string(),
        media_conf: MediaConfig::AvcConfig(AvcConfig {
            width: context.width() as u16,
            height: context.height() as u16,
            seq_param_set: vec![],
            pic_param_set: vec![],
        }),
    };
    mp4_writer
        .add_track(&track_config)
        .map_err(|e| e.to_string())?;
    let track_id = 1; // First track will have ID 1

    // 发送视频基本信息
    let video_info = serde_json::json!({
        "width": context.width(),
        "height": context.height(),
        "format": "rgb24"
    });
    app.emit_to(
        EventTarget::WebviewWindow {
            label: label.clone(),
        },
        "video-info",
        video_info,
    )
    .unwrap();

    // 解码并转换帧
    for (stream, packet) in ictx.packets() {
        if stream.index() == stream_index {
            let mut decoded = VideoFrame::empty();
            context.send_packet(&packet).map_err(|e| e.to_string())?;
            while context.receive_frame(&mut decoded).is_ok() {
                // 写入MP4片段
                let sample = Mp4Sample {
                    start_time: decoded.pts().unwrap_or(0) as u64,
                    duration: 0,
                    rendering_offset: 0,
                    is_sync: true,
                    bytes: decoded.data(0).to_vec().into(),
                };
                mp4_writer.write_sample(track_id, &sample).unwrap();

                // 获取MP4数据到buffer
                // Get the buffer from the writer
                let writer = mp4_writer.into_writer();
                let data = writer.get_ref();

                // 发送MP4片段
                app.emit_to(
                    EventTarget::WebviewWindow {
                        label: label.clone(),
                    },
                    "video-chunk",
                    data.to_vec(),
                )
                .unwrap();

                // 重新创建MP4写入器
                mp4_writer = Mp4Writer::write_start(writer, &config).map_err(|e| e.to_string())?;
            }
        }
    }
    mp4_writer.write_end().unwrap();

    Ok(())
}
