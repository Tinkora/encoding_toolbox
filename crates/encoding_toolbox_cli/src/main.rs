use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr as _;

use clap::{Args, Parser, Subcommand};
use encoding_toolbox_core::{
    DigestAlgorithm, Encoding, Error as CoreError, HmacAlgorithm, decode, digest, encode, hmac,
};
use serde::Serialize;

const CLI_MAX_INPUT_BYTES: u64 = 100 * 1024 * 1024;

#[derive(Debug, Parser)]
#[command(
    name = "tinkora-encoding",
    version,
    about = "Local encoding and digest toolbox"
)]
struct Cli {
    #[arg(long, global = true, help = "Emit a versioned JSON result")]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Encode(TransformArgs),
    Decode(TransformArgs),
    Digest(DigestArgs),
    Hmac(HmacArgs),
}

#[derive(Debug, Args)]
struct TransformArgs {
    #[arg(long)]
    algorithm: String,
    #[arg(default_value = "-")]
    input: PathBuf,
}

#[derive(Debug, Args)]
struct DigestArgs {
    #[arg(long)]
    algorithm: String,
    #[arg(default_value = "-")]
    input: PathBuf,
}

#[derive(Debug, Args)]
struct HmacArgs {
    #[arg(long)]
    algorithm: String,
    #[arg(long, value_name = "NAME")]
    key_env: String,
    #[arg(default_value = "-")]
    input: PathBuf,
}

#[derive(Debug)]
enum AppError {
    Core(CoreError),
    InputTooLarge,
    InvalidText,
    MissingHmacKey,
    InvalidHmacKey,
    Io,
    Usage(&'static str),
    Json,
}

impl AppError {
    const fn code(&self) -> &'static str {
        match self {
            Self::Core(error) => error.code(),
            Self::InputTooLarge => "INPUT_TOO_LARGE",
            Self::InvalidText => "INVALID_TEXT",
            Self::MissingHmacKey => "MISSING_HMAC_KEY",
            Self::InvalidHmacKey => "INVALID_HMAC_KEY",
            Self::Io => "IO_ERROR",
            Self::Usage(_) => "INVALID_USAGE",
            Self::Json => "JSON_ERROR",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Core(error) => error.to_string(),
            Self::InputTooLarge => {
                format!("input exceeds the {}-byte CLI limit", CLI_MAX_INPUT_BYTES)
            }
            Self::InvalidText => "encoded input must be UTF-8 text".to_owned(),
            Self::MissingHmacKey => "the named HMAC key environment variable is not set".to_owned(),
            Self::InvalidHmacKey => {
                "the HMAC key environment variable is not valid UTF-8".to_owned()
            }
            Self::Io => "input or output could not be read or written".to_owned(),
            Self::Usage(message) => (*message).to_owned(),
            Self::Json => "the JSON result could not be serialized".to_owned(),
        }
    }

    const fn exit_code(&self) -> u8 {
        if matches!(self, Self::Usage(_)) { 2 } else { 1 }
    }
}

impl From<CoreError> for AppError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}

#[derive(Serialize)]
struct JsonResult<'a> {
    schema_version: u8,
    operation: &'a str,
    algorithm: &'a str,
    result: &'a str,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error [{}]: {}", error.code(), error.message());
            ExitCode::from(error.exit_code())
        }
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Command::Encode(args) => {
            let algorithm = Encoding::from_str(&args.algorithm)?;
            let input = read_input(&args.input)?;
            let result = encode(algorithm, &input)?;
            write_text_result(cli.json, "encode", algorithm.key(), &result)
        }
        Command::Decode(args) => {
            if cli.json {
                return Err(AppError::Usage(
                    "decode writes arbitrary bytes and cannot be combined with --json",
                ));
            }
            let algorithm = Encoding::from_str(&args.algorithm)?;
            let input = read_input(&args.input)?;
            let text = std::str::from_utf8(&input).map_err(|_| AppError::InvalidText)?;
            let result = decode(algorithm, text.trim_ascii())?;
            io::stdout()
                .lock()
                .write_all(&result)
                .map_err(|_| AppError::Io)
        }
        Command::Digest(args) => {
            let algorithm = DigestAlgorithm::from_str(&args.algorithm)?;
            let input = read_input(&args.input)?;
            let result = digest(algorithm, &input);
            write_text_result(cli.json, "digest", algorithm.key(), &result)
        }
        Command::Hmac(args) => {
            let algorithm = HmacAlgorithm::from_str(&args.algorithm)?;
            let key = read_hmac_key(&args.key_env)?;
            let input = read_input(&args.input)?;
            let result = hmac(algorithm, key.as_bytes(), &input)?;
            write_text_result(cli.json, "hmac", algorithm.key(), &result)
        }
    }
}

fn read_input(path: &Path) -> Result<Vec<u8>, AppError> {
    if path == OsStr::new("-") {
        return read_bounded(io::stdin().lock());
    }

    let metadata = path.metadata().map_err(|_| AppError::Io)?;
    if metadata.len() > CLI_MAX_INPUT_BYTES {
        return Err(AppError::InputTooLarge);
    }
    let file = File::open(path).map_err(|_| AppError::Io)?;
    read_bounded(file)
}

fn read_bounded(reader: impl Read) -> Result<Vec<u8>, AppError> {
    read_bounded_with_limit(reader, CLI_MAX_INPUT_BYTES)
}

fn read_bounded_with_limit(reader: impl Read, max_bytes: u64) -> Result<Vec<u8>, AppError> {
    let mut input = Vec::new();
    reader
        .take(max_bytes + 1)
        .read_to_end(&mut input)
        .map_err(|_| AppError::Io)?;
    if input.len() as u64 > max_bytes {
        return Err(AppError::InputTooLarge);
    }
    Ok(input)
}

fn read_hmac_key(name: &str) -> Result<String, AppError> {
    std::env::var_os(name)
        .ok_or(AppError::MissingHmacKey)?
        .into_string()
        .map_err(|_| AppError::InvalidHmacKey)
}

fn write_text_result(
    json: bool,
    operation: &str,
    algorithm: &str,
    result: &str,
) -> Result<(), AppError> {
    let mut output = io::stdout().lock();
    if json {
        serde_json::to_writer(
            &mut output,
            &JsonResult {
                schema_version: 1,
                operation,
                algorithm,
                result,
            },
        )
        .map_err(|_| AppError::Json)?;
    } else {
        output
            .write_all(result.as_bytes())
            .map_err(|_| AppError::Io)?;
    }
    output.write_all(b"\n").map_err(|_| AppError::Io)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn bounded_reader_accepts_the_limit_and_rejects_one_more_byte() {
        assert_eq!(
            read_bounded_with_limit(Cursor::new(b"123"), 3).unwrap(),
            b"123"
        );
        let error = read_bounded_with_limit(Cursor::new(b"1234"), 3).unwrap_err();
        assert!(matches!(error, AppError::InputTooLarge));
    }
}
