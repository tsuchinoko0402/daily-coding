//! デイリーコーディングを実施するためのディレクトリを作成するツールです。
//! デイリーコーディング用のルートリポジトリで実行してください。
//! ルート配下の `workspace` ディレクトリ以下にディレクトリを作成します。

use chrono::Local;
use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[clap(
    name = "Create New Workspacce for Daily Coding",
    author = "OKAZAKI Shogo",
    version = "v1.0.0",
    about = "デイリーコーディングのためのワークスペースの作成をします。"
)]
struct AppArg {
    // 任意のオプション
    #[clap(short = 'd', long = "date")]
    date: Option<String>,

    #[clap(short = 't', long = "theme")]
    theme: Option<String>,

    // 必須のオプション
    language: String,
}

fn main() {
    let arg: AppArg = AppArg::parse();
    do_mkdir(
        arg.date
            .clone()
            .unwrap_or(Local::now().format("%Y-%m-%d").to_string()),
        arg.language,
        arg.theme,
    );
}

fn do_mkdir(date: String, language: String, theme: Option<String>) {
    let dir_name = match theme {
        Some(theme) => format!("workspace/{}_{}-{}", date, language, theme),
        None => format!("workspace/{}_{}", date, language),
    };
    match fs::create_dir_all(dir_name.clone()) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    };
    let dir_path = Path::new(&dir_name);
    assert!(env::set_current_dir(&dir_path).is_ok());
}
