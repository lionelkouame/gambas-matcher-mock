use crate::config::MockResponse;
use hyper::{Body, Response, StatusCode};
use std::fs;

pub struct MockGenerator;

impl MockGenerator {
    pub fn generate_response(
        mock: &MockResponse,
    ) -> Result<Response<Body>, Box<dyn std::error::Error>> {
        let mut response = Response::builder().status(StatusCode::from_u16(mock.status)?);

        // Add headers
        for (key, value) in &mock.headers {
            response = response.header(key, value);
        }

        // Get body content
        let body_content = if let Some(body_file) = &mock.body_file {
            // Read from file
            match fs::read_to_string(body_file) {
                Ok(content) => content,
                Err(e) => {
                    log::error!("Failed to read mock body file {}: {}", body_file, e);
                    return Err(Box::new(e));
                }
            }
        } else {
            // Use inline body
            mock.body.clone()
        };

        Ok(response.body(Body::from(body_content))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_generate_simple_response() {
        let mock = MockResponse {
            status: 200,
            headers: HashMap::new(),
            body: "Hello, World!".to_string(),
            body_file: None,
        };

        let result = MockGenerator::generate_response(&mock);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_generate_response_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let mock = MockResponse {
            status: 201,
            headers,
            body: r#"{"status": "created"}"#.to_string(),
            body_file: None,
        };

        let result = MockGenerator::generate_response(&mock);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );
    }
}
