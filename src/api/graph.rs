use oxigraph::model::NamedNode;
use salvo::prelude::*;
use salvo_oapi::endpoint;
use crate::db::oxigraph::Db;
use salvo_oapi::extract::{QueryParam};    
use oxigraph::model::NamedOrBlankNode;

#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to insert a graph into the Oxigraph store",
    description = "This endpoint accepts a JSON body with the graph data. It inserts the graph into the Oxigraph store and returns a success message."
)]
pub async fn create_graph(
    depot: &mut Depot,
    name: QueryParam<String, true>,
) -> Result<String, StatusError> {
    let db = depot.obtain::<Db>().unwrap();
    let graph = name.into_inner();

    //check if graph already exists
    if db.store.named_graphs().any(|res| res.ok().map_or(false, |g| g == NamedOrBlankNode::NamedNode(NamedNode::new(graph.clone()).unwrap()))) {
        return Err(StatusError::conflict());
    }

    let graph_name = NamedNode::new(graph).map_err(|_| StatusError::bad_request())?;

    db.store.insert_named_graph(&graph_name).map_err(|_| StatusError::internal_server_error())?;

    Ok("Graph created".into())
    
    
}

#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to delete a graph from the Oxigraph store",
    description = "This endpoint accepts a graph name as a query parameter. It deletes the graph from the Oxigraph store and returns a success message."
)]
pub async fn delete_graph(
    depot: &mut Depot,
    name: QueryParam<String, true>,
) -> Result<String, StatusError> {
    let db = depot.obtain::<Db>().unwrap();
    let graph = name.into_inner();
    let graph_name = NamedNode::new(graph).map_err(|_| StatusError::bad_request())?;
    
    // check if graph exists
    
    if !db.store.named_graphs().any(|res| res.ok().map_or(false, |g| g == NamedOrBlankNode::NamedNode(graph_name.clone()))) {
        return Err(StatusError::not_found());
    }
    db.store.remove_named_graph(&graph_name).map_err(|_| StatusError::internal_server_error())?;

    Ok("Graph deleted".into())
}

#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to get all the graphs from the Oxigraph store",
    description = "This endpoint returns a list of all the graph names in the Oxigraph store."
)]
pub async fn get_graphs(
    depot: &mut Depot,
) -> Result<Json<Vec<String>>, StatusError> {
    let db = depot.obtain::<Db>().unwrap();

    let graphs: Vec<String> = db.store.named_graphs()
        .filter_map(|res| res.ok().map(|graph| graph.to_string()))
        .collect();

    Ok(Json(graphs))
}