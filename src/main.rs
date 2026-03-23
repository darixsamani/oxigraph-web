use salvo::cors::AllowOrigin;
use salvo::http::Method;
use salvo::prelude::*;
use salvo_oapi::OpenApi;
use salvo::cors::Cors;
use crate::db::oxigraph::Db;
use crate::router::api_router;
pub mod db;
pub mod api;
pub mod router;
pub mod schemas;
pub mod services;

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt().init();


    let cors = Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers("content-type")
        .into_handler();

    let db = Db::new();

    let router = Router::new()
        .hoop(affix_state::inject(db))
        .push(api_router());

    let doc = OpenApi::new("A high-performance, full-featured web API for Oxigraph", "0.0.1").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/docs"));

     let service = Service::new(router).hoop(cors);

    let acceptor = TcpListener::new("0.0.0.0:8080").bind().await;

    Server::new(acceptor).serve(service).await;
}