use std::fmt;

#[derive(Debug)]
pub(crate) enum PatchError {
    Byte {
        offset: u32,
        actual: u8,
        expected: u8,
    },
    Dword {
        offset: u32,
        actual: u32,
        expected: u32,
    },
    Region(region::Error),
}

impl fmt::Display for PatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte {
                offset,
                actual,
                expected,
            } => write!(f, "{:#02x} != {:#02x} at {:#08x}", actual, expected, offset),
            Self::Dword {
                offset,
                actual,
                expected,
            } => write!(f, "{:#08x} != {:#08x} at {:#08x}", actual, expected, offset),
            Self::Region(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for PatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Byte { .. } => None,
            Self::Dword { .. } => None,
            Self::Region(e) => Some(e),
        }
    }
}

impl From<region::Error> for PatchError {
    fn from(value: region::Error) -> Self {
        Self::Region(value)
    }
}
