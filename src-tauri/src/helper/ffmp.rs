// use ffmpeg_next::{codec::Context as CodecContext, encoder::find as encoderFind, format::input, media::Type};
// use mp4::{AvcConfig, MediaConfig, Mp4Config, Mp4Sample, Mp4Writer, TrackConfig};
// use std::io::Cursor;
// use tauri::{AppHandle, Emitter, EventTarget};

// #[allow(unused)]
// pub fn decode_video_stream(app: AppHandle, path: &str, label: String) -> Result<(), String> {
//     ffmpeg_next::init().map_err(|e| e.to_string())?;

//     // 打开视频文件
//     let mut ictx = input(path).map_err(|e| e.to_string())?;
//     let stream = ictx
//         .streams()
//         .best(Type::Video)
//         .ok_or("No video stream found")?;
    
//     let stream_index = stream.index();

//     // 设置解码器
//     let encoder_codec = encoderFind(ffmpeg_next::codec::Id::H264).ok_or("No decoder found")?;
//     let context_decoder = CodecContext::new_with_codec(encoder_codec);
//     let mut decoder = context_decoder.decoder().video().map_err(|e| e.to_string())?;

//     // 每个片段的基本配置
//     let mp4_config = Mp4Config {
//         major_brand: str::parse("isom").unwrap(),
//         minor_version: 512,
//         compatible_brands: vec![
//             str::parse("isom").unwrap(),
//             str::parse("iso2").unwrap(),
//             str::parse("avc1").unwrap(),
//             str::parse("mp41").unwrap(),
//         ],
//         timescale: 1000,
//     };

//     let mut frame_count = 0;
//     let mut segment_data: Vec<u8> = Vec::new();

//     for (stream, packet) in ictx.packets() {
//         if stream.index() == stream_index {
//             decoder.send_packet(&packet).map_err(|e| e.to_string())?;
            
//             let mut decoded = ffmpeg_next::frame::Video::empty();
//             while decoder.receive_frame(&mut decoded).is_ok() {
//                 frame_count += 1;
                
//                 // 每30帧创建一个新片段
//                 if frame_count % 30 == 0 {
//                     let mut buffer = Vec::new();
//                     let writer = Cursor::new(&mut buffer);
//                     let mut mp4_writer = Mp4Writer::write_start(writer, &mp4_config)
//                         .map_err(|e| e.to_string())?;

//                     // 配置视频轨道
//                     let track_config = TrackConfig {
//                         track_type: mp4::TrackType::Video,
//                         timescale: 1000,
//                         language: "und".to_string(),
//                         media_conf: MediaConfig::AvcConfig(AvcConfig {
//                             width: decoder.width() as u16,
//                             height: decoder.height() as u16,
//                             seq_param_set: vec![],
//                             pic_param_set: vec![],
//                         }),
//                     };

//                     mp4_writer
//                         .add_track(&track_config)
//                         .map_err(|e| e.to_string())?;

//                     // 写入视频数据
//                     mp4_writer
//                         .write_sample(
//                             0,
//                             &Mp4Sample {
//                                 start_time: frame_count as u64,
//                                 duration: 40,
//                                 rendering_offset: 0,
//                                 is_sync: packet.is_key(),
//                                 bytes: packet.data().unwrap_or_default().to_vec().into(),
//                             },
//                         )
//                         .map_err(|e| e.to_string())?;

//                     mp4_writer.write_end().map_err(|e| e.to_string())?;

//                     // 发送片段到前端
//                     app.emit_to(
//                         EventTarget::WebviewWindow {
//                             label: label.clone(),
//                         },
//                         "video-chunk",
//                         buffer,
//                     )
//                     .map_err(|e| e.to_string())?;

//                     segment_data.clear();
//                 }
//             }
//         }
//     }

//     Ok(())
// }
