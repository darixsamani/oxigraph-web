use salvo::http::header;
use salvo::http::HeaderValue;
use salvo::prelude::*;
use salvo_oapi::endpoint;
use oxigraph::{io::RdfFormat, model::NamedNode};
use crate::services::export_service::export_graph;
use crate::db::oxigraph::Db;
use salvo_oapi::extract::QueryParam;
use oxigraph::model::GraphNameRef;

#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to export a graph from the Oxigraph store",
    description = "This endpoint accepts a graph name as a query parameter and an optional format query parameter. It exports the graph from the Oxigraph store in the specified format and returns it as a byte array."
)]
pub async fn export_graph_endpoint(
    depot: &mut Depot,
    res: &mut Response,
    graph: QueryParam<String, false>,
    format: QueryParam<String, false>,
) -> Result<(), StatusError> {

    let db = depot.obtain::<Db>().unwrap();
    let graph_name = graph.into_inner().unwrap_or_else(|| "".to_string());
    
    
    let rdf_format = match format.into_inner().unwrap_or_else(|| "turtle".to_string()).to_lowercase().as_str() {
        "turtle" | "ttl" => RdfFormat::Turtle,
        "ntriples" | "nt" => RdfFormat::NTriples,
        "nquads" | "nq" => RdfFormat::NQuads,
        "trig" => RdfFormat::TriG,
        "rdfxml" | "rdf" => RdfFormat::RdfXml,
        "jsonld" | "jld" => RdfFormat::JsonLd { profile: Default::default() },
        _ => return Err(StatusError::bad_request()),
    };


    let named_node_holder: Option<NamedNode>;

    let graph_name_ref: GraphNameRef = if graph_name.is_empty() {
        GraphNameRef::DefaultGraph
    } else {
        println!("Exporting graph: {}", graph_name);
        named_node_holder = Some(
            NamedNode::new(graph_name.as_str()).map_err(|e| {
                eprintln!("Invalid graph IRI '{}': {}", graph_name, e);
                StatusError::bad_request().brief(format!("Invalid graph IRI: '{}'", graph_name))
            })?
        );
        GraphNameRef::NamedNode(named_node_holder.as_ref().unwrap().as_ref())
    };

    let data = export_graph(&db.store, graph_name_ref, rdf_format)
        .map_err(|_| StatusError::internal_server_error())?;

    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    res.write_body(data).ok();
    

    Ok(())
}