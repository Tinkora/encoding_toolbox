use std::fs::{self, File};
use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const CLI_MAX_INPUT_BYTES: u64 = 100 * 1024 * 1024;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_tinkora-encoding")
}

fn run(args: &[&str], input: &[u8], environment: Option<(&str, &str)>) -> Output {
    let mut command = Command::new(binary());
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some((name, value)) = environment {
        command.env(name, value);
    }
    let mut child = command.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    if let Err(error) = stdin.write_all(input) {
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    drop(stdin);
    child.wait_with_output().unwrap()
}

fn temporary_path(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "tinkora-encoding-{label}-{}-{nonce}",
        std::process::id()
    ))
}

#[test]
fn stdin_and_file_inputs_produce_the_same_encoding() {
    let stdin = run(&["encode", "--algorithm", "base64"], b"hello", None);
    assert!(stdin.status.success());
    assert_eq!(stdin.stdout, b"aGVsbG8=\n");

    let path = temporary_path("input");
    fs::write(&path, b"hello").unwrap();
    let file = Command::new(binary())
        .args(["encode", "--algorithm", "base64"])
        .arg(&path)
        .output()
        .unwrap();
    fs::remove_file(path).unwrap();

    assert!(file.status.success());
    assert_eq!(file.stdout, stdin.stdout);
}

#[test]
fn decode_writes_binary_bytes_without_text_conversion() {
    let output = run(&["decode", "--algorithm", "base64"], b"AP+A", None);
    assert!(output.status.success());
    assert_eq!(output.stdout, [0x00, 0xff, 0x80]);
}

#[test]
fn json_output_has_a_versioned_machine_contract() {
    let output = run(&["digest", "--algorithm", "sha256", "--json"], b"abc", None);
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["operation"], "digest");
    assert_eq!(value["algorithm"], "sha256");
    assert_eq!(
        value["result"],
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn invalid_usage_exits_two() {
    let output = run(&["decode", "--algorithm", "base64", "--json"], b"", None);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}

#[test]
fn invalid_input_exits_one_with_a_stable_code() {
    let output = run(&["decode", "--algorithm", "hex"], b"abc", None);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("INVALID_ENCODING"));
}

#[test]
fn oversized_regular_file_is_rejected_before_reading() {
    let path = temporary_path("oversized");
    let file = File::create(&path).unwrap();
    file.set_len(CLI_MAX_INPUT_BYTES + 1).unwrap();
    let output = Command::new(binary())
        .args(["digest", "--algorithm", "sha256"])
        .arg(&path)
        .output()
        .unwrap();
    fs::remove_file(path).unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("INPUT_TOO_LARGE"));
}

#[test]
fn hmac_reads_only_an_environment_key_and_never_echoes_it() {
    let secret = "HMAC_TEST_SECRET_4fQ9";
    let success = run(
        &[
            "hmac",
            "--algorithm",
            "sha256",
            "--key-env",
            "TINKORA_TEST_HMAC_KEY",
        ],
        b"message",
        Some(("TINKORA_TEST_HMAC_KEY", secret)),
    );
    assert!(success.status.success());
    assert_eq!(success.stdout.len(), 65);
    assert!(!String::from_utf8_lossy(&success.stdout).contains(secret));
    assert!(!String::from_utf8_lossy(&success.stderr).contains(secret));

    let missing = run(
        &[
            "hmac",
            "--algorithm",
            "sha256",
            "--key-env",
            "TINKORA_MISSING_HMAC_KEY",
        ],
        b"message",
        None,
    );
    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("MISSING_HMAC_KEY"));
}

#[test]
fn command_helper_collects_output_when_the_process_exits_before_reading_stdin() {
    let input = vec![0_u8; 16 * 1024 * 1024];
    let output = run(
        &[
            "hmac",
            "--algorithm",
            "sha256",
            "--key-env",
            "TINKORA_MISSING_HMAC_KEY",
        ],
        &input,
        None,
    );

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("MISSING_HMAC_KEY"));
}
