use weichatrobot::{Image, Text};
use anyhow::Result;
use std::path::Path;


fn main() -> Result<()>{

	let image = Image::new(Path::new("./Cargo.toml"))?;
	println!("{}",serde_json::to_string(&image)?);

	let text = Text::new("你好", &[], &[])?;
	println!("{}",serde_json::to_string(&text)?);
	
	Ok(())	
}