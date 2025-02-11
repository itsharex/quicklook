// 使用 include! 宏包含生成的绑定
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
pub mod libarchive {
    include!(concat!(env!("OUT_DIR"), "/libarchive.rs"));
}