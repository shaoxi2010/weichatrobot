use std::{process::Command, io, io::Write};
use serde::Serialize;
use anyhow::Result;

pub fn post<T: Serialize>(url: &str, data: &T) -> Result<()> {
	let mut cmd = Command::new("curl");
	cmd.arg("-X").arg("POST");
	cmd.arg("-H").arg("Content-Type: application/json");
	cmd.arg("-d").arg(serde_json::to_string(data)?);
	cmd.arg(url);
	let output = cmd.output()?;
	io::stdout()
		.write_all(&output.stdout)?;
	io::stderr()
		.write_all(&output.stderr)?;
	Ok(())
}