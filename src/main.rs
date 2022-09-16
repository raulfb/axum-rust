use axum::{routing::get, Router};
use axum::handler::Handler;

#[tokio::main]
async fn main() {
    // Rutas
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", get(get_raiz))
        .route("/usuarios", get(get_usuarios))
        .route("/contacto", get(get_contacto));

    // Aplicacion corriendo en:  http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_raiz() -> String {
    String::from("raiz")
}
async fn get_usuarios() -> String {
    String::from("Ususarios")
}
async fn get_contacto() -> String {
    String::from("Contacto")
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {(
    axum::http::StatusCode::NOT_FOUND,
    format!("No existe la ruta: {}", uri),
)}
