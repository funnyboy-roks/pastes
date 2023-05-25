use std::path::PathBuf;

use clap::Parser;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Service {
    Bytebin,
    Pastes,
    Unset,
}

/// Pastes is a simple tool to upload files and text to https://pastes.dev or
/// https://bytebin.lucko.me.
///
/// To use, one need only to run: `pastes my_file.txt` and it will upload it to pastes.dev or `pastes
/// my_image.png` and it will upload it to bytebin.
///
/// Additionally, one can specify whether it should upload to pastes.dev or bytebin by using the
/// `--pastes` or `--bytebin` flags.  Note: These are mutually exclusive and will error if both are
/// used.
///
/// The intention is for this tool to work nicely in pipelines, as well as standalone, so
/// `cat my_file.txt | pastes` is equivalent to `pastes my_file.txt`
///
/// When using this in a script, I'd recommend using the `--json` flag with `jq`:
/// `echo "hello" | pastes --json | jq -r '.key'`
///
/// If you run into any issues or have a suggestion, please file an issue at
/// https://github.com/funnyboy-roks/pastes
#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// The file from which to read.  If not specified, then the program will read from STDIN
    pub file: Option<PathBuf>,

    /// Force the file to be uploaded to bytebin.lucko.me
    /// (conflicts with --pastes)
    #[arg(short, long, conflicts_with = "pastes")]
    pub bytebin: bool,

    /// Force the file to be uploaded to pastes.dev
    /// (conflicts with --bytebin)
    #[arg(short, long, conflicts_with = "bytebin")]
    pub pastes: bool,

    /// The `Content-Type` header to be set instead of guessing from the file extension
    #[arg(short = 't', long, value_name = "content-type")]
    pub content_type: Option<String>,

    /// The `User-Agent` header to use, overrides the config file
    #[arg(short, long, value_name = "user-agent")]
    pub user_agent: Option<String>,

    /// The path to the config file
    #[arg(short, long, value_name = "config")]
    pub config: Option<PathBuf>,

    /// Verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    // TODO:
    ///// When used, will treat the file name or STDIN as a key for pastes.dev or bytebin and print
    ///// it to STDOUT
    //#[arg(short, long)]
    //pub download: bool,
    /// Enable JSON output for the program results
    #[arg(short, long)]
    pub json: bool,
}

impl Cli {
    pub fn try_parse() -> anyhow::Result<Self> {
        let new = Self::parse();

        Ok(new)
    }

    pub fn dest(&self) -> Service {
        assert!(!(self.bytebin && self.pastes));
        match (self.bytebin, self.pastes) {
            (true, false) => Service::Bytebin,
            (false, true) => Service::Pastes,
            _ => Service::Unset,
        }
    }
}
