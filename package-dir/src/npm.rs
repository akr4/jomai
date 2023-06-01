use std::{ffi::OsStr, path::Path};

pub fn is_npm_package_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().file_name() == Some(OsStr::new("node_modules"))
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_npm_package_dir(path::Path::new("/a/b/node_modules")), true);
        assert_eq!(is_npm_package_dir(path::Path::new("/node_modules")), true);
        assert_eq!(is_npm_package_dir(path::Path::new("./node_modules")), true);
        assert_eq!(is_npm_package_dir(path::Path::new("../node_modules")), true);
        assert_eq!(is_npm_package_dir(path::Path::new("/a/b")), false);
    }
}
