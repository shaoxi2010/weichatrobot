use anyhow::{Ok, Result, Context};
use base64::{engine::general_purpose, Engine};
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::{collections::HashMap, path::Path};

#[derive(Serialize)]
pub struct Image {
    msgtype: &'static str,
    image: HashMap<&'static str, String>,
}

impl Image {
    pub fn new(img: &Path) -> Result<Self> {
        let mut image = HashMap::new();
        let mut data = Vec::new();

        let mut file = File::open(img)
			.context("Can not open image")?;
        file.read_to_end(&mut data)
			.context("Can not read image")?;
        image.insert("base64", general_purpose::STANDARD.encode(&data));
        image.insert("md5", format!("{:x}", md5::compute(data)));
        Ok(Self {
            msgtype: "image",
            image,
        })
    }
}

#[derive(Serialize)]
struct TextData {
    content: String,
    mentioned_list: Vec<String>,
    mentioned_mobile_list: Vec<String>,
}

#[derive(Serialize)]
pub struct Text {
    msgtype: &'static str,
    text: TextData,
}

impl Text {
    pub fn new(
        content: impl ToString,
        mentioned_list: &[impl ToString],
        mentioned_mobile_list: &[impl ToString],
    ) -> Result<Self> {
        Ok(Self {
            msgtype: "text",
            text: TextData {
                content: content.to_string(),
                mentioned_list: mentioned_list.into_iter().map(|x| x.to_string()).collect(),
                mentioned_mobile_list: mentioned_mobile_list
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            },
        })
    }
}
