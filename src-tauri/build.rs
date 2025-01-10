use std::env;

fn main() {
    // 设置环境变量
    env::set_var("VCPKGRS_DYNAMIC", "1");
    env::set_var("CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG", "true");

    // 设定 vcpkg 的安装目录（根据你安装 vcpkg 的路径来修改）
    let vcpkg_dir = env::var("VCPKG_ROOT").unwrap_or_default();
    println!("vcpkg_dir: {:?}", vcpkg_dir);
    println!("cargo:warning=Using VCPKG_ROOT: {}", vcpkg_dir);

    // 配置 vcpkg 环境
    println!("cargo:rerun-if-env-changed=VCPKG_ROOT");
    println!("cargo:rerun-if-changed=build.rs");

    // 使用 vcpkg 来查找和链接 FFmpeg 库
    // println!("cargo:rustc-link-search=native={}/installed/x64-windows/lib", vcpkg_dir);
    // println!("cargo:rustc-link-lib=dylib=avcodec");
    // println!("cargo:rustc-link-lib=dylib=avformat");
    // println!("cargo:rustc-link-lib=dylib=avutil");
    // 配置 FFmpeg
    #[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
    vcpkg::Config::new()
        .target_triplet("x64-windows")
        .emit_includes(true)
        .find_package("ffmpeg")
        .unwrap();
    #[cfg(all(target_arch = "aarch64", target_os = "windows", target_env = "msvc"))]
    vcpkg::Config::new()
        .target_triplet("arm64-windows")
        .emit_includes(true)
        .find_package("ffmpeg")
        .unwrap();

    tauri_build::build()
}
