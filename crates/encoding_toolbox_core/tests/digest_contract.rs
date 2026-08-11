use std::str::FromStr;

use encoding_toolbox_core::{DigestAlgorithm, HmacAlgorithm, digest, hmac};

#[test]
fn digests_match_published_known_answer_vectors() {
    let vectors = [
        (
            DigestAlgorithm::Sha256,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            DigestAlgorithm::Sha384,
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7",
        ),
        (
            DigestAlgorithm::Sha512,
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        ),
        (
            DigestAlgorithm::Blake3,
            "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
        ),
        (DigestAlgorithm::Md5, "900150983cd24fb0d6963f7d28e17f72"),
    ];

    for (algorithm, expected) in vectors {
        assert_eq!(digest(algorithm, b"abc"), expected);
    }
}

#[test]
fn digest_accepts_the_empty_standard_vector() {
    assert_eq!(
        digest(DigestAlgorithm::Sha256, b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn hmac_matches_rfc_4231_test_case_one() {
    let key = [0x0b; 20];
    assert_eq!(
        hmac(HmacAlgorithm::Sha256, &key, b"Hi There").unwrap(),
        "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
    );
    assert_eq!(
        hmac(HmacAlgorithm::Sha512, &key, b"Hi There").unwrap(),
        concat!(
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde",
            "daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
        )
    );
}

#[test]
fn hmac_accepts_empty_messages_but_rejects_empty_keys() {
    assert_eq!(hmac(HmacAlgorithm::Sha256, b"key", b"").unwrap().len(), 64);
    let error = hmac(HmacAlgorithm::Sha256, b"", b"message").unwrap_err();
    assert_eq!(error.code(), "EMPTY_HMAC_KEY");
}

#[test]
fn algorithm_keys_round_trip_and_md5_is_marked_legacy() {
    for algorithm in DigestAlgorithm::ALL {
        assert_eq!(
            DigestAlgorithm::from_str(algorithm.key()).unwrap(),
            *algorithm
        );
    }
    for algorithm in HmacAlgorithm::ALL {
        assert_eq!(
            HmacAlgorithm::from_str(algorithm.key()).unwrap(),
            *algorithm
        );
    }
    assert!(DigestAlgorithm::Md5.is_legacy());
    assert!(!DigestAlgorithm::Sha256.is_legacy());
}
