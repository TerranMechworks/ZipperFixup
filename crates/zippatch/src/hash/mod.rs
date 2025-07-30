use hex_literal::hex;
use std::fmt;

pub(crate) struct KnownBinary {
    exe: &'static str,
    lang: &'static str,
    ver: &'static str,
    quirks: &'static str,
}

impl fmt::Display for KnownBinary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (VERSION: {}, LANG: {}, QUIRKS: {})",
            self.exe, self.lang, self.ver, self.quirks
        )
    }
}

pub(crate) enum HashCheck {
    Known(KnownBinary),
    Unknown(String),
}

pub(crate) fn hash_binary(contents: &[u8]) -> HashCheck {
    use sha2::Digest as _;
    let digest: [u8; 32] = sha2::Sha256::digest(contents).into();
    match &digest {
        MECH3_EN_12_STD_HASH => HashCheck::Known(MECH3_EN_12_STD),
        MECH3_EN_11_STD_HASH => HashCheck::Known(MECH3_EN_11_STD),
        MECH3_EN_10_STD_HASH => HashCheck::Known(MECH3_EN_10_STD),
        MECH3_DE_10_STD_HASH => HashCheck::Known(MECH3_DE_10_STD),
        MECH3_DE_10_P1_HASH => HashCheck::Known(MECH3_DE_10_P1),
        hash => HashCheck::Unknown(hex(hash)),
    }
}

fn hex(hash: &[u8; 32]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(64);
    for b in hash {
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests;

const MECH3_EN_12_STD: KnownBinary = KnownBinary {
    exe: "Mech3.exe",
    lang: "en",
    ver: "1.2",
    quirks: "standard",
};
const MECH3_EN_12_STD_HASH: &[u8; 32] =
    &hex!("95bc2c114c9b2e5c5ada8e40ca78cc79470f862e45313ffad0d1e8e6d4d916bf");

const MECH3_EN_11_STD: KnownBinary = KnownBinary {
    exe: "Mech3.exe",
    lang: "en",
    ver: "1.1",
    quirks: "standard",
};
const MECH3_EN_11_STD_HASH: &[u8; 32] =
    &hex!("8e1f86c84abed8eacf048842a97c2ebe3e847ecff3fb7e1f8c2500b4edcda15b");

const MECH3_EN_10_STD: KnownBinary = KnownBinary {
    exe: "Mech3.exe",
    lang: "en",
    ver: "1.0",
    quirks: "standard",
};
const MECH3_EN_10_STD_HASH: &[u8; 32] =
    &hex!("1c6419fcdbb0503d2dee006861477fc7354039e6ce95c431b5ac5e4995b465bc");

const MECH3_DE_10_STD: KnownBinary = KnownBinary {
    exe: "Mech3.exe",
    lang: "de",
    ver: "1.0",
    quirks: "standard",
};
const MECH3_DE_10_STD_HASH: &[u8; 32] =
    &hex!("5213c1cf2f6713b5069a2718ce6030ba0fea3f8cf346447aaf6381d807986af0");

const MECH3_DE_10_P1: KnownBinary = KnownBinary {
    exe: "Mech3.exe",
    lang: "de",
    ver: "1.0",
    quirks: "patched",
};
const MECH3_DE_10_P1_HASH: &[u8; 32] =
    &hex!("cb7130aec692a74ecd275a8d98936c6c1f9f86a7ebb1504794284f95ae9c2e95");
