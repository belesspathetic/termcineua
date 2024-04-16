use std::{
    fs::{create_dir, File},
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use tempdir::TempDir;

pub fn main_prompt() -> String {
    println!("Welcome to TermCineUA");
    println!("Please, choose an option:\n1. Cartoons\n0. Update config (beta)");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub async fn watch(stream: String) -> anyhow::Result<()> {
    let temp_dir = TempDir::new("cartoon_")?;
    let path = temp_dir.path().join("cartoon.m3u8");
    get_m3u8(stream, &path).await?;

    println!("Відтворюю...");
    let _ = Command::new("vlc").arg(path).output()?;

    Ok(())
}

pub async fn download(stream: String) -> anyhow::Result<()> {
    let temp_dir = TempDir::new("cartoon_")?;
    let m3u8_path = temp_dir.path().join("cartoon.m3u8");
    let output_path = PathBuf::from("./output");
    let mp4_path = PathBuf::from("./output/output.mp4");

    if !output_path.exists() {
        create_dir(&output_path)?;
    }

    get_m3u8(stream.clone(), &m3u8_path).await?;
    println!("Завантажую...");
    let output = Command::new("ffmpeg")
        .arg("-protocol_whitelist")
        .arg("file,http,https,tcp,tls,crypto")
        .arg("-i")
        .arg(&m3u8_path)
        .arg("-c")
        .arg("copy")
        .arg("-bsf:a")
        .arg("aac_adtstoasc")
        .arg(&mp4_path)
        .output()?;

    println!("{:?}", output);

    Ok(())
}

pub async fn get_m3u8(stream: String, path: &PathBuf) -> anyhow::Result<()> {
    let mut file = File::create(&path)?;
    let cl = reqwest::Client::new();

    let mut resp = cl.get(&stream).send().await?;

    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}
