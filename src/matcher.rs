use crate::config::{Matcher, Rule};
use hyper::{Body, Request};
use regex::Regex;
use std::collections::HashMap;

pub struct RequestMatcher {
    rules: Vec<Rule>,
}

impl RequestMatcher {
    pub fn new(rules: Vec<Rule>) -> Self {
        RequestMatcher { rules }
    }

    pub async fn find_matching_rule(&self, req: &Request<Body>) -> Option<Rule> {
        for rule in &self.rules {
            if self.matches_request(&rule.matcher, req).await {
                return Some(rule.clone());
            }
        }
        None
    }

    async fn matches_request(&self, matcher: &Matcher, req: &Request<Body>) -> bool {
        // Match method
        if let Some(method) = &matcher.method {
            if req.method().as_str().to_uppercase() != method.to_uppercase() {
                return false;
            }
        }

        // Match exact path
        if let Some(path) = &matcher.path {
            if req.uri().path() != path {
                return false;
            }
        }

        // Match path with regex
        if let Some(path_regex) = &matcher.path_regex {
            if let Ok(re) = Regex::new(path_regex) {
                if !re.is_match(req.uri().path()) {
                    return false;
                }
            } else {
                log::warn!("Invalid regex pattern: {}", path_regex);
                return false;
            }
        }

        // Match headers
        if let Some(required_headers) = &matcher.headers {
            for (key, value) in required_headers {
                if let Some(header_value) = req.headers().get(key) {
                    match header_value.to_str() {
                        Ok(header_str) => {
                            if header_str != value {
                                return false;
                            }
                        }
                        Err(_) => {
                            log::warn!("Failed to convert header '{}' value to string", key);
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
        }

        // Match query parameters
        if let Some(required_params) = &matcher.query_params {
            let query_params = parse_query_string(req.uri().query().unwrap_or(""));
            for (key, value) in required_params {
                if query_params.get(key) != Some(value) {
                    return false;
                }
            }
        }

        true
    }
}

fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_string() {
        let query = "foo=bar&baz=qux";
        let params = parse_query_string(query);
        assert_eq!(params.get("foo"), Some(&"bar".to_string()));
        assert_eq!(params.get("baz"), Some(&"qux".to_string()));
    }

    #[test]
    fn test_parse_query_string_empty() {
        let query = "";
        let params = parse_query_string(query);
        assert_eq!(params.len(), 0);
    }
}
