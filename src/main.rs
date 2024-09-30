use reqwest;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {

    const SPOTIFY_DOWNLOAD_STANDARD: &str = "https://download.scdn.co/SpotifySetup.exe";

    println!("程式已经启动");

    if detect_standard_spotify_executeable() {
        println!("检测到Spotify客户端，绕过下载");
        patch_spotx();
    } else {
        println!("未检测到Spotify客户端，开始自动下载");
        download_spotify_client_setup(&SPOTIFY_DOWNLOAD_STANDARD);
        println!("下载已完成，开始安装");
        setup_spotify_client();
        patch_spotx();
    }

}

fn detect_standard_spotify_executeable() -> bool {

    let appdata = match env::var("APPDATA") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error retrieving APPDATA: {}", e);
            return false;
        }
    };

    let standard_spotify_executable = format!("{}/Spotify/spotify.exe", appdata);

    if Path::new(&standard_spotify_executable).exists() {
        return true
    } else {
        return false
    }
}

fn download_spotify_client_setup(url: &str) {
    let mut resp = reqwest::blocking::get(url).unwrap();

    let mut file = File::create("spotify_setup.exe").unwrap();
    
    std::io::copy(&mut resp, &mut file).unwrap();

}

fn setup_spotify_client() {
    let setup_package = "./spotify_setup.exe";

    let output = Command::new(setup_package)
        .output()
        .expect("无法打开安装包，检查问题，或者告诉我");
     
    if !output.status.success() {
        eprintln!("安装包无法正常执行，如果出现了这个问题，删掉安装包再执行一遍，或者是自己手动安装吧，是软件内部返回的错误报告引起的，这个消息只是提醒")
    }
}

fn patch_spotx() {
    println!("正在开始从SpotX@GitHub项目拉取代码部署，请确保网络通畅！");
    let mut resp = reqwest::blocking::get("https://raw.githack.com/amd64fox/SpotX/main/scripts/Install_Auto.bat").unwrap();
    let mut file = File::create("spotx_install.bat").unwrap();
    std::io::copy(&mut resp, &mut file).unwrap();

    println!("成功！正在部署SpotX!");

    let spotx = "./spotx_install.bat";

    let output = Command::new(spotx)
        .output()
        .expect("无法执行...可能是哪里出问题惹！！是不是没有spotx_install.bat这个文件还是没权限啦！ \\>A</");

    if !output.status.success() {
        eprintln!("无法部署...到底是怎么回事啦！！它返回了什么报错告诉我！！！ >A<!!");
    } else {
        println!("不出意外的话~可以用了哦~xwx")
    }
}