use std::path::Path;

/// Returns true if the given path is Composer's vendor directory.
/// This tests if the path is `vendor` and there is a `composer.json` in the parent directory.
pub fn is_composer_package_dir<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    // ensure the path is "vendor"
    if let Some(file_name) = path.file_name() {
        if file_name != "vendor" {
            return false;
        }
    } else {
        return false;
    }

    let parent = path.parent();

    if parent.is_none() {
        return false;
    }

    let parent = parent.unwrap();

    parent.join("composer.json").is_file()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_true() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("vendor")).unwrap();
        let vendor_dir = dir.path().join("vendor");
        fs::File::create(dir.path().join("composer.json")).unwrap();

        assert_eq!(is_composer_package_dir(&vendor_dir), true);
    }

    #[test]
    fn no_composer_json() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("vendor")).unwrap();
        let vendor_dir = dir.path().join("vendor");

        assert_eq!(is_composer_package_dir(&vendor_dir), false);
    }

    #[test]
    fn not_vendor_dir() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(is_composer_package_dir(&dir.path()), false);
    }
}
