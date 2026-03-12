

use salvo_oapi::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TripleInput {
    pub object: String,
    pub predicate: String,
    pub subject: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SparqlQuery {
    pub query: String,
}