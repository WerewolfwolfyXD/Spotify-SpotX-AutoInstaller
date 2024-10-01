use reqwest;
use std::{env, io};
use std::fs::File;
use std::path::Path;
use std::process::{exit, Command};

use unic_langid::LanguageIdentifier;
use fluent::{FluentBundle, FluentResource, FluentError};


fn main() {

    // i18n Manager

    static FTL_DATA_DEFAULT: &'static str = include_str!("../localize/fluent/en_us.ftl");
    static FTL_DATA_ZH_CN: &'static str = include_str!("../localize/fluent/zh_cn.ftl");

    let ftl_lang_data = match locale_config::Locale::current().to_string().split(',').next().unwrap() {
        "zh-CN" => FTL_DATA_ZH_CN,
        "en-US" => FTL_DATA_DEFAULT,
        _ => FTL_DATA_DEFAULT
    };

    let res = FluentResource::try_new(String::from(ftl_lang_data))
        .expect("Cannot load localize language file!");

    let langid_en: LanguageIdentifier = "en-US".parse().expect("Cannot parse language identifier!");
    let langid_cn: LanguageIdentifier = "zh-CN".parse().expect("Cannot parse language identifier!");
    let mut bundle = FluentBundle::new(vec![langid_cn, langid_en]);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to a bundle!");

    let mut errors: Vec<FluentError> = vec![];

    // i18n End


    const SPOTIFY_DOWNLOAD_STANDARD: &str = "https://download.scdn.co/SpotifySetup.exe";

    println!("{}",
             bundle.format_pattern(&bundle.get_message("program-started").unwrap().value().unwrap(), None, &mut errors)
    );

    if detect_standard_spotify_execution() {
        println!("{}",
                 bundle.format_pattern(&bundle.get_message("msg-detected-available-spotify-skip-download").unwrap().value().unwrap(), None, &mut errors));
        patch_spotx(&mut bundle);
    } else {
        println!("{}",
                 bundle.format_pattern(&bundle.get_message("msg-non-detected-available-spotify-go-download").unwrap().value().unwrap(), None, &mut errors));
        download_spotify_client_setup(&SPOTIFY_DOWNLOAD_STANDARD);
        println!("{}",
                 bundle.format_pattern(&bundle.get_message("msg-spotify-client-downloaded-2-setup").unwrap().value().unwrap(), None, &mut errors));
        setup_spotify_client(&mut bundle);
        patch_spotx(&mut bundle);
    }

    exit_program(&mut bundle);
}

fn detect_standard_spotify_execution() -> bool {

    let appdata = match env::var("APPDATA") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error retrieving APPDATA: {}", e);
            return false;
        }
    };

    let standard_spotify_executable = format!("{}/Spotify/spotify.exe", appdata);

    return Path::new(&standard_spotify_executable).exists()
}

fn download_spotify_client_setup(url: &str) {
    let mut resp = reqwest::blocking::get(url).unwrap();

    let mut file = File::create("spotify_setup.exe").unwrap();
    
    std::io::copy(&mut resp, &mut file).unwrap();

}

fn setup_spotify_client(bundle: &FluentBundle<FluentResource>) {
    let mut errors: Vec<FluentError> = vec![];


    let setup_package = "./spotify_setup.exe";

    let output = Command::new(setup_package)
        .output()
        .expect("Cannot run the installation!");
     
    if !output.status.success() {
        eprintln!("{}",
                  bundle.format_pattern(&bundle.get_message("err-setup-spotify-client-output-status-fail").unwrap().value().unwrap(), None, &mut errors));
    }
}

fn patch_spotx(bundle: &FluentBundle<FluentResource>) {
    let mut errors: Vec<FluentError> = vec![];


    println!("{}",
             bundle.format_pattern(&bundle.get_message("msg-fetch-spotx-from-github").unwrap().value().unwrap(), None, &mut errors));
    let mut resp = reqwest::blocking::get("https://raw.githack.com/amd64fox/SpotX/main/scripts/Install_Auto.bat").unwrap();
    let mut file = File::create("spotx_install.bat").unwrap();
    std::io::copy(&mut resp, &mut file).unwrap();

    println!("{}",
             bundle.format_pattern(&bundle.get_message("msg-success--to-setup").unwrap().value().unwrap(), None, &mut errors));

    let spotx_installer = "./spotx_install.bat";

    let output = Command::new(spotx_installer)
        .output()
        .expect("无法执行...可能是哪里出问题惹！！是不是没有spotx_install.bat这个文件还是没权限啦！ \\>A</");

    if !output.status.success() {
        eprintln!("{}",
                  bundle.format_pattern(&bundle.get_message("err-cannot-deploy-spotx").unwrap().value().unwrap(), None, &mut errors));
    } else {
        println!("{}",
                 bundle.format_pattern(&bundle.get_message("msg-enjoy").unwrap().value().unwrap(), None, &mut errors));
    }
}

fn exit_program(bundle: &FluentBundle<FluentResource>) {
    let mut errors: Vec<FluentError> = vec![];

    let mut input = String::new();
    println!("{}",
             bundle.format_pattern(&bundle.get_message("msg-enter-2-exit").unwrap().value().unwrap(), None, &mut errors));
    io::stdin().read_line(&mut input).expect("Nothing especially thing will happened, to a result, it quit. Isn't it?");
    exit(0);
}