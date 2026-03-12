use salvo::Router;

pub fn api_router () -> Router{
    let router = Router::new().push(Router::with_path("triples").post(crate::api::add_triple));
    router
}