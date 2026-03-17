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
    name: QueryParam<String, false>,
    format: QueryParam<String, false>,
) -> Result<(), StatusError> {
    
    let db = depot.obtain::<Db>().unwrap();
    let graph_name = name.into_inner().unwrap_or_else(|| "default".to_string());
    print!("before format parsing");
    let rdf_format = match format.into_inner().unwrap_or_else(|| "turtle".to_string()).to_lowercase().as_str() {
        "turtle" | "ttl" => RdfFormat::Turtle,
        "ntriples" | "nt" => RdfFormat::NTriples,
        "nquads" | "nq" => RdfFormat::NQuads,
        "trig" => RdfFormat::TriG,
        "rdfxml" | "rdf" => RdfFormat::RdfXml,
        "jsonld" | "jld" => RdfFormat::JsonLd { profile: Default::default() },
        _ => return Err(StatusError::bad_request()),
    };
    print!("after format parsing");

    let named = NamedNode::new(graph_name.as_str()).map_err(|_| StatusError::bad_request())?;
    let data = export_graph(&db.store, GraphNameRef::NamedNode(named.as_ref()), rdf_format).map_err(|_| StatusError::internal_server_error())?;

    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    res.write_body(data).ok();
    Ok(())
}