use std::{ffi::OsStr, path::Path};

pub fn is_bower_package_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().file_name() == Some(OsStr::new("bower_components"))
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_bower_package_dir(path::Path::new("/a/b/bower_components")), true);
        assert_eq!(is_bower_package_dir(path::Path::new("/bower_components")), true);
        assert_eq!(is_bower_package_dir(path::Path::new("./bower_components")), true);
        assert_eq!(is_bower_package_dir(path::Path::new("../bower_components")), true);
        assert_eq!(is_bower_package_dir(path::Path::new("/a/b")), false);
    }
}
