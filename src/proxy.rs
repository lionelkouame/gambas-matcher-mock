use crate::config::Config;
use crate::matcher::RequestMatcher;
use crate::mock::MockGenerator;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct ProxyServer {
    config: Arc<Config>,
    matcher: Arc<RequestMatcher>,
}

impl ProxyServer {
    pub fn new(config: Config) -> Self {
        let matcher = RequestMatcher::new(config.rules.clone());
        ProxyServer {
            config: Arc::new(config),
            matcher: Arc::new(matcher),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = format!("{}:{}", self.config.server.host, self.config.server.port)
            .parse()?;

        log::info!("Starting Gambas Matcher Mock server on {}", addr);
        log::info!("Loaded {} rules", self.config.rules.len());

        let config = self.config.clone();
        let matcher = self.matcher.clone();

        let make_svc = make_service_fn(move |_conn| {
            let config = config.clone();
            let matcher = matcher.clone();

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let config = config.clone();
                    let matcher = matcher.clone();
                    handle_request(req, config, matcher)
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);

        log::info!("Server running successfully");

        if let Err(e) = server.await {
            log::error!("Server error: {}", e);
        }

        Ok(())
    }
}

async fn handle_request(
    req: Request<Body>,
    _config: Arc<Config>,
    matcher: Arc<RequestMatcher>,
) -> Result<Response<Body>, Infallible> {
    log::info!(
        "Received request: {} {}",
        req.method(),
        req.uri().path()
    );

    // Try to find a matching rule
    match matcher.find_matching_rule(&req).await {
        Some(rule) => {
            log::info!("Matched rule: {}", rule.name);
            
            match MockGenerator::generate_response(&rule.response) {
                Ok(response) => {
                    log::info!(
                        "Returning mock response with status {}",
                        rule.response.status
                    );
                    Ok(response)
                }
                Err(e) => {
                    log::error!("Error generating mock response: {}", e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(format!("Mock generation error: {}", e)))
                        .unwrap())
                }
            }
        }
        None => {
            log::warn!("No matching rule found for: {} {}", req.method(), req.uri().path());
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("No matching rule found"))
                .unwrap())
        }
    }
}
