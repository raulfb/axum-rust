use axum::{routing::get, Router};
use axum::handler::Handler;

#[tokio::main]
async fn main() {
    // Rutas
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", get(get_raiz))
        .route("/usuarios", get(get_usuarios))
        .route("/usuario/:id", get(get_usuario))
        .route("/contacto", get(get_contacto))
        .route("/estado", get(get_con_estado));

    // Aplicacion corriendo en:  http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_con_estado()->(axum::http::StatusCode,String){(
    axum::http::StatusCode::OK,String::from("Todo correcto")
)}

pub async fn get_raiz() -> String {
    String::from("raiz")
}

pub async fn get_usuario(axum::extract::Path(id):axum::extract::Path<String>) -> String {
    format!("Usuario con id {}",id)
}

pub async fn get_usuarios() -> String {
    String::from("Ususarios")
}

pub async fn get_contacto() -> String {
    String::from("Contacto")
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {(
    axum::http::StatusCode::NOT_FOUND,
    format!("No existe la ruta: {}", uri),
)}
