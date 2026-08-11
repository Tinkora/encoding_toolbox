use encoding_toolbox_core::{Encoding, MAX_DECODE_TEXT_BYTES, decode, encode};

#[test]
fn empty_input_uses_the_standard_empty_vector() {
    for algorithm in [
        Encoding::Base64,
        Encoding::Base64Url,
        Encoding::Base32,
        Encoding::Base32Hex,
        Encoding::Hex,
    ] {
        assert_eq!(encode(algorithm, b"").unwrap(), "");
        assert_eq!(decode(algorithm, "").unwrap(), b"");
    }
}

#[test]
fn base64_matches_rfc_4648_vectors() {
    let vectors = [
        (b"f".as_slice(), "Zg=="),
        (b"fo".as_slice(), "Zm8="),
        (b"foo".as_slice(), "Zm9v"),
        (b"foobar".as_slice(), "Zm9vYmFy"),
    ];

    for (plain, encoded) in vectors {
        assert_eq!(encode(Encoding::Base64, plain).unwrap(), encoded);
        assert_eq!(decode(Encoding::Base64, encoded).unwrap(), plain);
    }
}

#[test]
fn base64url_decodes_padded_and_unpadded_canonical_input() {
    assert_eq!(encode(Encoding::Base64Url, b"\xfb\xff").unwrap(), "-_8");
    assert_eq!(decode(Encoding::Base64Url, "-_8").unwrap(), b"\xfb\xff");
    assert_eq!(decode(Encoding::Base64Url, "-_8=").unwrap(), b"\xfb\xff");
}

#[test]
fn hexadecimal_decode_accepts_mixed_case_and_rejects_odd_length() {
    assert_eq!(decode(Encoding::Hex, "aB01").unwrap(), b"\xab\x01");
    let error = decode(Encoding::Hex, "abc").unwrap_err();
    assert_eq!(error.code(), "INVALID_ENCODING");
}

#[test]
fn base32_requires_the_canonical_uppercase_alphabet() {
    assert_eq!(encode(Encoding::Base32, b"foo").unwrap(), "MZXW6===");
    let error = decode(Encoding::Base32, "mzxw6===").unwrap_err();
    assert_eq!(error.code(), "INVALID_ENCODING");
}

#[test]
fn every_byte_round_trips_through_every_encoding() {
    let bytes: Vec<u8> = (u8::MIN..=u8::MAX).collect();
    for algorithm in Encoding::ALL {
        let encoded = encode(*algorithm, &bytes).unwrap();
        assert_eq!(decode(*algorithm, &encoded).unwrap(), bytes);
    }
}

#[test]
fn decode_rejects_input_above_the_documented_limit() {
    let input = "A".repeat(MAX_DECODE_TEXT_BYTES + 1);
    let error = decode(Encoding::Base64, &input).unwrap_err();
    assert_eq!(error.code(), "INPUT_TOO_LARGE");
}
