use std::{ffi::OsStr, path::Path};

pub fn is_cocoapods_pods_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().file_name() == Some(OsStr::new("Pods"))
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_cocoapods_pods_dir(path::Path::new("/a/b/Pods")), true);
        assert_eq!(is_cocoapods_pods_dir(path::Path::new("/Pods")), true);
        assert_eq!(is_cocoapods_pods_dir(path::Path::new("/a/b")), false);
    }
}
