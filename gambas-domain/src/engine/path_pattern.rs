#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPattern(String);

impl PathPattern {
    pub fn new(path: &str) -> Self {
        if path.starts_with("/") {
            Self(path.to_string())
        } else {
            Self(format!("/{}", path))
        }
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_keep_leading_slash_is_present() {
        let path: PathPattern = PathPattern::new("/foo/bar");
        assert_eq!(path.value(), "/foo/bar");
    }

    #[test]
    fn should_add_leading_slash_is_missing() {
        let path: PathPattern = PathPattern::new("foo/bar");
        assert_eq!(path.value(), "/foo/bar");
    }

    #[test]
    fn should_be_equal_if_paths_are_identical() {
        let path1 = PathPattern::new("/test");
        let path2 = PathPattern::new("/test");
        assert_eq!(path1, path2);
    }
}
