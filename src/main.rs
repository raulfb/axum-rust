#[tokio::main]
async fn main() {

    use axum::{Router, routing::get};

    // Rutas
    let app = Router::new()
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
async fn get_usuarios()->String {
    String::from("Ususarios")

}
async fn get_contacto()-> String {
    String::from("Contacto")
}
