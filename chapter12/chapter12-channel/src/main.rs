use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    sync::mpsc::channel,
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

    let mut handles = vec![];
    let mut channels = vec![];
    let (counter_tx, counter_rx) = channel::<usize>();

    // 受信側 = サムネイル作成処理側 の立ち上げ
    for _ in 0..4 {
        let (tx, rx) = channel::<PathBuf>();
        channels.push(tx);
        let counter_tx = counter_tx.clone();
        let output = args.output.clone();
        handles.push(thread::spawn(move || {
            while let Ok(path) = rx.recv() {
                let output_path = output.join(path.file_name().unwrap());
                let img = image::open(&path);
                if let Ok(img) = img {
                    let thumbnail = img.thumbnail(64, 64);
                    thumbnail.save(output_path).unwrap();

                    counter_tx.send(1).unwrap();
                }
            }
        }));
    }

    // 送信側は画像ファイルのパスを送信する
    for (index, item) in read_dir(&args.input).unwrap().enumerate() {
        let item = item.unwrap();
        let path = item.path();
        if path.is_dir() {
            continue;
        }
        channels[index % channels.len()].send(path).unwrap();
    }

    // 処理の完了通知
    for channel in channels {
        drop(channel);
    }
    drop(counter_tx);

    println!("Processed {} images", counter_rx.iter().count());
}
