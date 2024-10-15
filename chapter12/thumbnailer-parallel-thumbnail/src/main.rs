use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
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
    let mut all_paths = vec![];
    for entry in read_dir(&args.input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // フォルダは処理しない
            continue;
        }
        all_paths.push(path);
    }

    let processed_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for chunk in all_paths.chunks((all_paths.len() + 3) / 4) {
        let chunk = chunk.to_vec();
        let processed_count = processed_count.clone();
        let output = args.output.clone();
        handles.push(thread::spawn(move || {
            for path in chunk {
                let output_path = output.join(path.file_name().unwrap());
                let img = image::open(&path); // 画像ファイルの読み込み
                if let Ok(img) = img {
                    let thumbnail = img.thumbnail(64, 64); // サムネイル化
                    thumbnail.save(output_path).unwrap(); // ファイル保存
                    let mut writer = processed_count.lock().unwrap();
                    *writer += 1;
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Processed {} images",
        processed_count.as_ref().lock().unwrap()
    );
}
