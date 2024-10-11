use crate::error::Error;
use crate::error::Result;
use binrw::{binrw, BinRead, BinWrite};
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::{env, fs};
pub static CFG: LazyLock<Config> = LazyLock::new(Config::init);

#[binrw]
#[brw(big)]
pub struct Config {
    header: [u8; 5],
    timestamp: i64,
    token_len: u32,

    // wrapping_add/wrapping_sub简单的移位混淆
    #[br(count = token_len, map = |s:Vec<u8>| String::from_utf8(s.iter().map(|c| c.wrapping_add(5)).collect()).unwrap() )]
    #[bw(map = |s| s.clone().into_bytes().iter().map(|c|c.wrapping_sub(5)).collect::<Vec<u8>>() )]
    pub token: String,
}

impl Config {
    pub fn init() -> Self {
        let mut file = fs::File::open(get_file_path()).unwrap_or_else(|_| {
            eprintln!("{}", Error::UnAuth);
            std::process::exit(1)
        });
        Config::read(&mut file).unwrap()
    }

    pub fn clear() -> Result<()> {
        let path = get_dir_path();

        if path.exists() {
            fs::remove_dir_all(path).map_err(Error::from)
        } else {
            Ok(())
        }
    }

    pub fn new(token: &str) -> Self {
        let token = token.to_string();
        let version = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();

        Config {
            header: [b'A', b'U', b'T', b'H', version],
            timestamp: Utc::now().timestamp_millis(),
            token_len: token.len() as u32,
            token,
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = get_file_path();

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();

        self.write(&mut file).unwrap();

        Ok(())
    }

    // fn check(&self) -> Result<()> {
    //     // 以后也许会有一些扩展
    //     Ok(())
    // }
}

fn get_dir_path() -> PathBuf {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap();
    Path::new(&home_dir).join(".blc")
}

fn get_file_path() -> PathBuf {
    let path = get_dir_path();
    let config_file_path: std::path::PathBuf = path.join("blc.b");

    if !path.exists() {
        fs::create_dir_all(&path).expect("初始化配置文件失败");
    }
    config_file_path
}
