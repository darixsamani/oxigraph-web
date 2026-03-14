use salvo::Router;
use crate::api::graph::{create_graph, delete_graph, get_graphs};
use crate::api::triple::add_triple;
use crate::api::sparql::{sparql, sparql_get};
use crate::api::import::import_pdf;

pub fn api_router () -> Router{
    let router = Router::new()
                        .push(Router::with_path("triples").post(add_triple))
                        .push(Router::with_path("sparql").post(sparql).get(sparql_get))
                        .push(
                            Router::with_path("graphs")
                            .post(create_graph)
                            .delete(delete_graph)
                            .get(get_graphs)
                        )
                        .push(Router::with_path("rdf/import").post(import_pdf));
    router
}