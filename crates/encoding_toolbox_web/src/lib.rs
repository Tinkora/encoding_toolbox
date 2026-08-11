//! Structured WASM facade for the Encoding Toolbox browser application.

use std::str::FromStr as _;

use encoding_toolbox_core::{
    DigestAlgorithm, Encoding, Error as CoreError, HmacAlgorithm, decode, digest, encode, hmac,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

pub const WEB_TEXT_MAX_BYTES: usize = 1024 * 1024;
pub const WEB_FILE_MAX_BYTES: usize = 20 * 1024 * 1024;

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

#[derive(Serialize)]
struct TextResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ErrorBody>,
}

#[derive(Serialize)]
struct BytesResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ErrorBody>,
}

#[derive(Serialize)]
struct AlgorithmEntry {
    key: &'static str,
    label: &'static str,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    legacy: bool,
}

#[derive(Serialize)]
struct AlgorithmMetadata {
    encodings: Vec<AlgorithmEntry>,
    digests: Vec<AlgorithmEntry>,
    hmacs: Vec<AlgorithmEntry>,
}

#[wasm_bindgen]
pub fn wasm_encode_text(algorithm: &str, input: &str) -> JsValue {
    if let Err(error) = validate_size(input.len(), WEB_TEXT_MAX_BYTES) {
        return text_error(error);
    }
    let response = Encoding::from_str(algorithm)
        .and_then(|algorithm| encode(algorithm, input.as_bytes()))
        .map(text_success)
        .unwrap_or_else(text_error_response);
    serialize(&response)
}

#[wasm_bindgen]
pub fn wasm_encode_bytes(algorithm: &str, input: &[u8]) -> JsValue {
    if let Err(error) = validate_size(input.len(), WEB_FILE_MAX_BYTES) {
        return text_error(error);
    }
    let response = Encoding::from_str(algorithm)
        .and_then(|algorithm| encode(algorithm, input))
        .map(text_success)
        .unwrap_or_else(text_error_response);
    serialize(&response)
}

#[wasm_bindgen]
pub fn wasm_decode_text(algorithm: &str, input: &str) -> JsValue {
    if let Err(error) = validate_size(input.len(), WEB_TEXT_MAX_BYTES) {
        return bytes_error(error);
    }
    let response = Encoding::from_str(algorithm)
        .and_then(|algorithm| decode(algorithm, input))
        .map(|bytes| BytesResponse {
            ok: true,
            bytes: Some(bytes),
            error: None,
        })
        .unwrap_or_else(bytes_error_response);
    serialize(&response)
}

#[wasm_bindgen]
pub fn wasm_digest_text(algorithm: &str, input: &str) -> JsValue {
    if let Err(error) = validate_size(input.len(), WEB_TEXT_MAX_BYTES) {
        return text_error(error);
    }
    digest_response(algorithm, input.as_bytes())
}

#[wasm_bindgen]
pub fn wasm_digest_bytes(algorithm: &str, input: &[u8]) -> JsValue {
    if let Err(error) = validate_size(input.len(), WEB_FILE_MAX_BYTES) {
        return text_error(error);
    }
    digest_response(algorithm, input)
}

#[wasm_bindgen]
pub fn wasm_hmac_text(algorithm: &str, key: &str, message: &str) -> JsValue {
    if let Err(error) = validate_size(message.len(), WEB_TEXT_MAX_BYTES) {
        return text_error(error);
    }
    hmac_response(algorithm, key.as_bytes(), message.as_bytes())
}

#[wasm_bindgen]
pub fn wasm_hmac_bytes(algorithm: &str, key: &str, message: &[u8]) -> JsValue {
    if let Err(error) = validate_size(message.len(), WEB_FILE_MAX_BYTES) {
        return text_error(error);
    }
    hmac_response(algorithm, key.as_bytes(), message)
}

#[wasm_bindgen]
pub fn wasm_algorithm_metadata() -> JsValue {
    let metadata = AlgorithmMetadata {
        encodings: Encoding::ALL
            .iter()
            .map(|algorithm| AlgorithmEntry {
                key: algorithm.key(),
                label: algorithm.label(),
                legacy: false,
            })
            .collect(),
        digests: DigestAlgorithm::ALL
            .iter()
            .map(|algorithm| AlgorithmEntry {
                key: algorithm.key(),
                label: algorithm.label(),
                legacy: algorithm.is_legacy(),
            })
            .collect(),
        hmacs: HmacAlgorithm::ALL
            .iter()
            .map(|algorithm| AlgorithmEntry {
                key: algorithm.key(),
                label: algorithm.label(),
                legacy: false,
            })
            .collect(),
    };
    serialize(&metadata)
}

fn digest_response(algorithm: &str, input: &[u8]) -> JsValue {
    let response = DigestAlgorithm::from_str(algorithm)
        .map(|algorithm| text_success(digest(algorithm, input)))
        .unwrap_or_else(text_error_response);
    serialize(&response)
}

fn hmac_response(algorithm: &str, key: &[u8], message: &[u8]) -> JsValue {
    let response = HmacAlgorithm::from_str(algorithm)
        .and_then(|algorithm| hmac(algorithm, key, message))
        .map(text_success)
        .unwrap_or_else(text_error_response);
    serialize(&response)
}

fn validate_size(actual: usize, maximum: usize) -> Result<(), CoreError> {
    if actual > maximum {
        return Err(CoreError::InputTooLarge { max_bytes: maximum });
    }
    Ok(())
}

fn text_success(result: String) -> TextResponse {
    TextResponse {
        ok: true,
        result: Some(result),
        error: None,
    }
}

fn text_error_response(error: CoreError) -> TextResponse {
    TextResponse {
        ok: false,
        result: None,
        error: Some(error_body(error)),
    }
}

fn bytes_error_response(error: CoreError) -> BytesResponse {
    BytesResponse {
        ok: false,
        bytes: None,
        error: Some(error_body(error)),
    }
}

fn error_body(error: CoreError) -> ErrorBody {
    ErrorBody {
        code: error.code(),
        message: error.to_string(),
    }
}

fn text_error(error: CoreError) -> JsValue {
    serialize(&text_error_response(error))
}

fn bytes_error(error: CoreError) -> JsValue {
    serialize(&bytes_error_response(error))
}

fn serialize(value: &impl Serialize) -> JsValue {
    serde_wasm_bindgen::to_value(value).expect("static WASM response must serialize")
}
