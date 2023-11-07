use anyhow::{Context, Result};
use mktemp::Temp;
use serde::Serialize;
use std::{fs::OpenOptions, io, io::Write, process::Command};

pub fn post<T: Serialize>(url: &str, data: &T) -> Result<()> {
    let data = serde_json::to_string(data).context("Post serialized failed")?;
    let tmp = Temp::new_file().context("Post make tempfile failed")?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&tmp)
        .context("Post open tempfiled failed")?;
    file.write_all(data.as_bytes())
        .context("Post write tempfile failed")?;
    file.flush().context("Post flush failed")?;
    file.sync_data().context("Post sync data failed")?;

    let mut cmd = Command::new("curl");
    cmd.arg("-X").arg("POST");
    cmd.arg("-H").arg("Content-Type: application/json");
    cmd.arg("-d").arg(format!("@{}", tmp.as_path().display()));
    cmd.arg(url);
    let output = cmd.output().context("Post execute curl failed")?;
    io::stdout()
        .write_all(&output.stdout)
        .context("curl stdout error")?;
    io::stderr()
        .write_all(&output.stderr)
        .context("curl stderr error")?;
    Ok(())
}
