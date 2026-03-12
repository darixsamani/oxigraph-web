use salvo::Router;
use crate::api::triple::add_triple;
use crate::api::sparql::{sparql, sparql_get};


pub fn api_router () -> Router{
    let router = Router::new()
                        .push(Router::with_path("triples").post(add_triple))
                        .push(Router::with_path("sparql").post(sparql).get(sparql_get));
    router
}