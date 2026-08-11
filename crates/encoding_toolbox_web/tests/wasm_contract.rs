#![cfg(target_arch = "wasm32")]

use encoding_toolbox_web::{
    wasm_algorithm_metadata, wasm_decode_text, wasm_digest_text, wasm_encode_text, wasm_hmac_text,
};
use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

fn property(value: &JsValue, name: &str) -> JsValue {
    Reflect::get(value, &JsValue::from_str(name)).unwrap()
}

#[wasm_bindgen_test]
fn encoding_and_decode_return_structured_results() {
    let encoded = wasm_encode_text("base64", "hello");
    assert_eq!(property(&encoded, "ok"), JsValue::TRUE);
    assert_eq!(
        property(&encoded, "result").as_string().unwrap(),
        "aGVsbG8="
    );

    let decoded = wasm_decode_text("base64", "AP+A");
    assert_eq!(property(&decoded, "ok"), JsValue::TRUE);
    let bytes = Array::from(&property(&decoded, "bytes"));
    assert_eq!(bytes.length(), 3);
    assert_eq!(bytes.get(0).as_f64(), Some(0.0));
    assert_eq!(bytes.get(1).as_f64(), Some(255.0));
    assert_eq!(bytes.get(2).as_f64(), Some(128.0));
}

#[wasm_bindgen_test]
fn digest_and_hmac_share_the_core_known_answers() {
    let digest = wasm_digest_text("sha256", "abc");
    assert_eq!(
        property(&digest, "result").as_string().unwrap(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );

    let mac = wasm_hmac_text(
        "sha256",
        "key",
        "The quick brown fox jumps over the lazy dog",
    );
    assert_eq!(
        property(&mac, "result").as_string().unwrap(),
        "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
    );
}

#[wasm_bindgen_test]
fn text_input_limit_returns_a_stable_secret_free_error() {
    let secret = "TEXT_LIMIT_SECRET_9qX";
    let input = format!("{}{}", "x".repeat(1024 * 1024), secret);
    let response = wasm_encode_text("base64", &input);
    assert_eq!(property(&response, "ok"), JsValue::FALSE);
    let error = property(&response, "error");
    assert_eq!(
        property(&error, "code").as_string().unwrap(),
        "INPUT_TOO_LARGE"
    );
    assert!(
        !js_sys::JSON::stringify(&response)
            .unwrap()
            .as_string()
            .unwrap()
            .contains(secret)
    );
}

#[wasm_bindgen_test]
fn invalid_input_and_empty_hmac_keys_use_stable_codes() {
    let invalid = wasm_decode_text("hex", "abc");
    assert_eq!(
        property(&property(&invalid, "error"), "code")
            .as_string()
            .unwrap(),
        "INVALID_ENCODING"
    );

    let empty_key = wasm_hmac_text("sha256", "", "message");
    assert_eq!(
        property(&property(&empty_key, "error"), "code")
            .as_string()
            .unwrap(),
        "EMPTY_HMAC_KEY"
    );
}

#[wasm_bindgen_test]
fn metadata_marks_md5_as_legacy_and_lists_each_operation() {
    let metadata = wasm_algorithm_metadata();
    let digests = Array::from(&property(&metadata, "digests"));
    let md5 = digests
        .iter()
        .find(|entry| property(entry, "key").as_string().as_deref() == Some("md5"))
        .unwrap();
    assert_eq!(property(&md5, "legacy"), JsValue::TRUE);
    assert_eq!(Array::from(&property(&metadata, "encodings")).length(), 5);
    assert_eq!(Array::from(&property(&metadata, "hmacs")).length(), 2);
}
