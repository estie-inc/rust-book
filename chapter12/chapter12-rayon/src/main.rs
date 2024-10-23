use clap::Parser;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    // サムネイル化する元画像が入っているフォルダ
    input: PathBuf,
    // サムネイルにした画像を保存するフォルダ
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    // 出力先フォルダの作成
    create_dir_all(&args.output).unwrap();

    // 処理対象ファイルの列挙
    let items: Vec<_> = read_dir(&args.input).unwrap().collect();
    let result = items.into_par_iter().map(|item| {
        let item = item.unwrap();
        let path = item.path();
        let output_path = args.output.join(path.file_name().unwrap());
        let img = image::open(&path);
        if let Ok(img) = img {
            let thumbnail = img.thumbnail(64, 64);
            thumbnail.save(output_path).unwrap();
            1
        } else {
            0
        }
    });

    println!("Processed {} images", result.sum::<u32>());
}
