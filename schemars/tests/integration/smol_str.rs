use smol_str03::SmolStr;

use crate::prelude::*;

#[test]
fn smol_str() {
    test!(SmolStr)
        .assert_identical::<String>()
        .assert_allows_ser_roundtrip(["".into(), "test".into()])
        .assert_matches_de_roundtrip(arbitrary_values());
}
