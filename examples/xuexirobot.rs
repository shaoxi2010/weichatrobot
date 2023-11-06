use anyhow::Result;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use serde::Deserialize;
use std::io::{prelude::*, BufReader};
use std::{fs::read_dir, fs::File, path::Path};
use weichatrobot::{post, Image, Text};

#[derive(Deserialize)]
struct Token {
    url: String,
    dry_run: bool,
}

#[derive(Deserialize)]
struct MessageConfig {
    text: String,
	mentioned_list: Vec<String>,
	mentioned_mobile_list: Vec<String>,
    picture: String,
}

#[derive(Deserialize)]
struct RobotConfig {
    token: Token,
    message: MessageConfig,
}

fn select_random_index(len: usize) -> usize {
    let between = Uniform::new(0, len);
    let mut rng = thread_rng();
    between.sample(&mut rng)
}

fn send_message(config: &RobotConfig) -> Result<()> {
    let message = Path::new(&config.message.text);
    if message.exists() {
        let file = File::open(message)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        let mut messages = Vec::new();
        while let Ok(n) = reader.read_line(&mut buffer) {
            if n == 0 {
                break;
            }
            messages.push(buffer.trim_end().to_string());
            buffer.clear();
        }
        let select = select_random_index(messages.len());
        let message = Text::new(&messages[select], &config.message.mentioned_list, &config.message.mentioned_mobile_list)?;
        if config.token.dry_run {
            println!("{}", serde_json::to_string(&message)?);
        } else {
            post(&config.token.url, &message)?;
        }
    }
    Ok(())
}

fn send_picture(config: &RobotConfig) -> Result<()> {
    let picture = Path::new(&config.message.picture);
    if picture.is_dir() {
        let pictures = read_dir(picture)?
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();
        let select = select_random_index(pictures.len());
        let picture = Image::new(&pictures[select].path())?;
        if config.token.dry_run {
            println!("{}", serde_json::to_string(&picture)?);
        } else {
            post(&config.token.url, &picture)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut config = File::open(Path::new("config.toml"))?;
    let mut config_string = String::new();
    config.read_to_string(&mut config_string)?;
    let config: RobotConfig = toml::from_str(&config_string)?;
    send_message(&config)?;
    send_picture(&config)?;
    Ok(())
}
