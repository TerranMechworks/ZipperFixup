use super::{MECH3_EN_12_STD_HASH, hex};

#[test]
fn hash_hex() {
    let actual = hex(MECH3_EN_12_STD_HASH);
    assert_eq!(
        actual,
        "95bc2c114c9b2e5c5ada8e40ca78cc79470f862e45313ffad0d1e8e6d4d916bf"
    );
}
