use std::path::Path;

pub fn is_python_package_dir<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    // ensure the path is "site-packages"
    if let Some(file_name) = path.file_name() {
        if file_name != "site-packages" {
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

    if !parent.is_dir() {
        return false;
    }

    return if let Some(file_name) = parent.file_name() {
        file_name.to_string_lossy().starts_with("python")
    } else {
        false
    };
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_true() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("python3.6/site-packages")).unwrap();
        let package_dir = dir.path().join("python3.6/site-packages");

        assert_eq!(is_python_package_dir(&package_dir), true);
    }

    #[test]
    fn not_python_dir() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("aaa/site-packages")).unwrap();
        let package_dir = dir.path().join("aaa/site-packages");

        assert_eq!(is_python_package_dir(&package_dir), false);
    }

    #[test]
    fn not_site_package_dir() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(is_python_package_dir(&dir.path()), false);
    }
}
