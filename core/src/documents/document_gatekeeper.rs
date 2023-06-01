use std::env::VarError;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DocumentGatekeeper {
    data_dir: PathBuf,
}

impl DocumentGatekeeper {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub fn is_eligible<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        path.is_file()
            && path.extension().map(|ext| ext == "md").unwrap_or(false)
            && !is_under_package_dir(path)
            && !is_under_data_dir(path, &self.data_dir)
            && !is_hidden(path)
            && !(is_under_library_dir(path) && !is_mobile_documents(path))
    }

    pub fn is_eligible_if_file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        path.extension().map(|ext| ext == "md").unwrap_or(false)
            && !is_under_package_dir(path)
            && !is_under_data_dir(path, &self.data_dir)
            && !is_hidden(path)
            && !(is_under_library_dir(path) && !is_mobile_documents(path))
    }
}

fn is_under_package_dir<P: AsRef<Path>>(path: P) -> bool {
    use package_dir::*;

    let is_package_dir = is_npm_package_dir(&path)
        || is_bower_package_dir(&path)
        || is_python_package_dir(&path)
        || is_bundler_package_dir(&path)
        || is_composer_package_dir(&path)
        || is_chef_cookbook_dir(&path)
        || is_cocoapods_pods_dir(&path);

    if is_package_dir {
        true
    } else {
        match path.as_ref().parent() {
            Some(parent) => is_under_package_dir(parent),
            None => false,
        }
    }
}

pub fn is_under_data_dir<P: AsRef<Path>, Q: AsRef<Path>>(path: P, data_dir: Q) -> bool {
    path.as_ref().starts_with(data_dir)
}

/// macOS
fn is_mobile_documents<P: AsRef<Path>>(path: P) -> bool {
    let home_dir = match std::env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(_) => return false,
    };
    let mobile_documents_dir = Path::new(&home_dir).join("Library").join("Mobile Documents");
    path.as_ref().starts_with(mobile_documents_dir)
}

/// macOS
fn is_under_library_dir<P: AsRef<Path>>(path: P) -> bool {
    let home_dir = match std::env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(_) => return false,
    };
    let library_dir = Path::new(&home_dir).join("Library");
    path.as_ref().starts_with(library_dir)
}

pub fn is_hidden<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .file_name()
        .map(|file_name| file_name.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Setup local test environment
    #[ignore]
    #[test]
    fn test_is_ok_but_under_library_dir() {
        fn is_ok<P: AsRef<Path>>(path: P) -> bool {
            let path = path.as_ref();
            !(is_under_library_dir(path) && !is_mobile_documents(path))
        }
        assert!(!is_ok(
            "/Users/akira/Library/Application Support/Code/User/History/a583de1/dY2H.md"
        ));
        assert!(is_ok(
            "/Users/akira/Library/Mobile Documents/iCloud~md~obsidian/Documents/main/Untitled.md"
        ))
    }
}
