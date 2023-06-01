use std::path::Path;

/// Returns true if `a` is a parent of `b`.
pub fn is_parent<P: AsRef<Path>, Q: AsRef<Path>>(a: P, b: Q) -> bool {
    let a = a.as_ref();
    let b = b.as_ref();
    if a == b {
        return false;
    }
    let mut a = a.components();
    let mut b = b.components();
    loop {
        match (a.next(), b.next()) {
            (Some(a), Some(b)) => {
                if a != b {
                    return false;
                }
            }
            (None, Some(_)) => return true,
            (Some(_), None) => return false,
            (None, None) => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_parent() {
        assert!(is_parent("/a/b/c", "/a/b/c/d"), "immediate parent");
        assert!(is_parent("/a/b", "/a/b/c/d"), "ancestor");
        assert!(!is_parent("/a/b/c", "/a/b/c"), "same path");
        assert!(!is_parent("/a/b/c", "/a/b"));
    }
}
