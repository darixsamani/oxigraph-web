use oxigraph::model::*;
use oxigraph::store::Store;

use crate::schemas::triples::TripleInput;

pub fn insert_triple(store: &Store, data: TripleInput) -> anyhow::Result<()> {
    let triple = Triple::new(
        NamedNode::new(data.subject)?,
        NamedNode::new(data.predicate)?,
        NamedNode::new(data.object)?,
    );

    let quad = Quad::new(
        triple.subject.clone(),
        triple.predicate.clone(),
        triple.object.clone(),
        GraphName::DefaultGraph,
    );

    store.insert(&quad)?;
    Ok(())
}