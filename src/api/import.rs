use oxigraph::{io::{RdfFormat, RdfParser}, model::{NamedNode}};
use salvo::{prelude::*};
use salvo_oapi::{endpoint, extract::QueryParam};

use crate::db::oxigraph::Db;


#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to import a graph into the Oxigraph store",
    description = "This endpoint accepts a JSON body with the graph data. It imports the graph into the Oxigraph store and returns a success message."
)]
pub async fn import_pdf(
    req: &mut Request,
    depot: &mut Depot,
    format: QueryParam<String, true>,
    graph: QueryParam<String, false>,
    
) -> Result<String, StatusError> {
    let db = depot.obtain::<Db>().unwrap();
    let body = req.payload().await.map_err(|_| StatusError::bad_request())?;

    // also detect rdf_format from content-type header if format query parameter is not provided
    
    let rdf_format = {
        match format.into_inner().to_lowercase().as_str() {
            "turtle" | "ttl" => RdfFormat::Turtle,
            "ntriples" | "nt" => RdfFormat::NTriples,
            "nquads" | "nq" => RdfFormat::NQuads,
            "trig" => RdfFormat::TriG,
            "rdfxml" | "rdf" => RdfFormat::RdfXml,
            "jsonld" | "jld" => RdfFormat::JsonLd { profile: Default::default() },
            _ => return Err(StatusError::bad_request()),
        }
    };

    if let Some(graph_name) = graph.as_ref() {
        println!("Importing graph into named graph: {}", graph_name);
        let named_graph = NamedNode::new(graph_name)
            .map_err(|_| StatusError::bad_request())?;
        db.store.load_from_slice(
            RdfParser::from_format(rdf_format).with_default_graph(named_graph),
            body.as_ref(),
        ).map_err(|_| StatusError::bad_request())?;
    } else {
        db.store.load_from_slice( rdf_format, body.as_ref()).map_err(|_| StatusError::bad_request())?;
    }

    Ok("Graph imported".into())
}