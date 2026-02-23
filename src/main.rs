use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 1. Définir l'application avec une route explicite pour "/"
    let app = Router::new()
        .route("/", get(|| async { "Hello Gambas Matcher Mock is alive!" }));

    // 2. Définir l'adresse (0.0.0.0 est le plus sûr sous WSL)
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server started on http://localhost:3000");

    // 3. Lancer le serveur
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}