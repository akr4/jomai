use std::{ffi::OsStr, path::Path};

fn normalize_string<S: AsRef<str>>(s: S) -> String {
    use unicode_normalization::UnicodeNormalization;
    s.as_ref().nfc().collect()
}

pub trait PathStringNormalizationExt {
    fn to_normalized_path_string(&self) -> String;
}

impl PathStringNormalizationExt for &Path {
    fn to_normalized_path_string(&self) -> String {
        normalize_string(self.to_string_lossy())
    }
}

impl PathStringNormalizationExt for &OsStr {
    fn to_normalized_path_string(&self) -> String {
        normalize_string(self.to_string_lossy())
    }
}
