use oxigraph::io::RdfFormat;
use oxigraph::model::GraphNameRef;
use oxigraph::store::Store;

pub fn export_graph(store: &Store, graph_name: GraphNameRef, format: RdfFormat) -> Result<Vec<u8>, String> {
    let mut buffer = Vec::new();
    store.dump_graph_to_writer(graph_name, format, &mut buffer).map_err(|e| e.to_string())?;
    Ok(buffer)
}