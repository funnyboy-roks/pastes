use anyhow::Context;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{stdin, stdout, Read, Write},
    str::FromStr,
};

mod cli;
mod config;

use cli::Cli;

use crate::{cli::Service, config::Config};

#[derive(Debug, PartialEq, Eq)]
struct UrlSet {
    /// Post to this url to upload something
    post: &'static str,
    /// Get from the url (with the key after) to get a file
    api: &'static str,
    /// Get from the url (with the key after) to get the editor
    gui: &'static str,
}

pub const DEFAULT_USERAGENT: &str = "pastes by funnyboy_roks <funnyboyroks@github.com>";

const BYTEBIN: UrlSet = UrlSet {
    post: "https://bytebin.lucko.me/post",
    api: "https://bytebin.lucko.me/",
    gui: "https://bytebin.lucko.me/",
};

const PASTES: UrlSet = UrlSet {
    post: "https://api.pastes.dev/post",
    api: "https://api.pastes.dev/",
    gui: "https://pastes.dev/",
};

#[derive(Debug, Clone, Deserialize)]
struct PostResponse {
    key: String,
}

#[derive(Debug, Clone, Serialize)]
struct ProgramOutput {
    key: String,
    service: cli::Service,
    url: String,
    zipped: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;
    let config = Config::load_config(cli.config.clone())?;

    let data = if let Some(ref file) = cli.file {
        fs::read(file).context("Invalid file path")?
    } else {
        let mut buf = Vec::new();
        let bytes = stdin()
            .read_to_end(&mut buf)
            .context("Unable to read from STDIN")?;
        eprintln!("Read {} bytes from STDIN", bytes);
        buf
    };

    // TODO: Add Tokio for nicer file streams

    let mimetype = if let Some(ref mt) = cli.content_type {
        // This will parse the mime type even if it's not "valid", but that's okay, since the user
        // knows best
        mt.parse()
            .context("Unable to parse provided content type")?
    } else if cli.file.is_none() {
        eprintln!("Content type not specified when using STDIN, so using 'text/plain'");
        mime::TEXT_PLAIN
    } else {
        let file = cli.file.clone().expect("checked above");
        let mimetype = mime_guess::from_path(&file).first();

        if let Some(mt) = mimetype {
            eprintln!("Using mimetype {} from file extension.", mt);
            mt
        } else {
            eprintln!("Unable to guess mimetype from file extension, using 'text/plain'.");
            mime::Mime::from_str(
                &cli.content_type
                    .clone()
                    .unwrap_or_else(|| "text/plain".into()),
            )
            .context("Unable to parse mime from configuration.")?
        }
    };

    let url_set = match cli.dest() {
        Service::Pastes => PASTES,
        Service::Bytebin => BYTEBIN,
        Service::Unset => match (mimetype.type_(), mimetype.subtype()) {
            (mime::TEXT, _) => PASTES,
            (mime::APPLICATION, mime::JAVASCRIPT) | (mime::APPLICATION, mime::JSON) => PASTES,
            // TODO: Add more types?
            _ => BYTEBIN,
        },
    };

    eprintln!("Zipping data...");
    let mut e = flate2::write::GzEncoder::new(Vec::new(), Compression::default());

    let (data, zipped) = if let Err(err) = e.write_all(&data) {
        eprintln!("Unable to zip data: {}", err);
        (data, false)
    } else {
        match e.finish() {
            Err(err) => {
                eprintln!("Unable to zip data: {}", err);
                (data, false)
            }
            Ok(zipped_data) => {
                eprintln!("Zipped into {} bytes", zipped_data.len());
                // Since zipping doesn't work on smalller files, we don't always zip.
                if zipped_data.len() > data.len() {
                    eprintln!(
                        "Since the zipped data was larger than unzipped, sending unzipped data."
                    );
                    (data, false)
                } else {
                    (zipped_data, true)
                }
            }
        }
    };

    eprintln!("Uploading...");
    let client = reqwest::blocking::Client::new();

    let mut req = client
        .post(url_set.post)
        .body(data)
        .header("content-type", mimetype.essence_str())
        .header("user-agent", cli.user_agent.unwrap_or(config.user_agent));

    if zipped {
        req = req.header("content-encoding", "gzip");
    }

    if let Some(headers) = config.headers {
        for (key, value) in headers {
            match &*key.to_lowercase() {
                // These are specific headers, so ignore them
                "content-type" | "user-agent" | "content-encoding" => {}
                _ => req = req.header(key, value),
            }
        }
    }

    let res = req.send().context("Error contacting api")?;

    let PostResponse { key } = res
        .json()
        .context("Unable to parse json response from api.")?;

    if cli.json {
        let prog_out = ProgramOutput {
            key: key.clone(),
            service: if url_set == PASTES {
                Service::Pastes
            } else {
                Service::Bytebin
            },
            url: format!("{}{}", url_set.gui, key),
            zipped,
        };

        serde_json::to_writer(stdout(), &prog_out).context("Error writing json value to STDOUT")?;
    } else {
        println!("File uploaded to {}{}", url_set.gui, key);
    }

    Ok(())
}
