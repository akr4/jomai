use std::{ffi::OsStr, path::Path};

pub fn is_chef_cookbook_dir<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.file_name() == Some(OsStr::new("cookbooks")) || path.file_name() == Some(OsStr::new("site-cookbooks"))
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_chef_cookbook_dir(path::Path::new("/a/b/cookbooks")), true);
        assert_eq!(is_chef_cookbook_dir(path::Path::new("/cookbooks")), true);
        assert_eq!(is_chef_cookbook_dir(path::Path::new("/site-cookbooks")), true);
        assert_eq!(is_chef_cookbook_dir(path::Path::new("/a/b")), false);
    }
}
