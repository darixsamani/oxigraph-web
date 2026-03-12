
use salvo::prelude::*;
use salvo_oapi::endpoint;
use crate::db::oxigraph::Db;
use salvo_oapi::extract::{JsonBody, QueryParam};
use crate::services::sparql_service::execute_query;

use crate::schemas::triples::SparqlQuery;


#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to execute a SPARQL query against the Oxigraph store",
    description = "This endpoint accepts a JSON body with a SPARQL query. It executes the query against the Oxigraph store and returns the results in JSON format."
)]
pub async fn sparql(
    depot: &mut Depot,
    data: JsonBody<SparqlQuery> 
) ->  Result<Json<serde_json::Value>, StatusError> {
    let db = depot.obtain::<Db>().unwrap();

    let results = execute_query(&db.store, &data.into_inner().query);
    Ok(Json(results))
}

#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to execute a SPARQL query against the Oxigraph store using GET method",
    description = "This endpoint accepts a SPARQL query as a query parameter. It executes the query against the Oxigraph store and returns the results in JSON format."
)]
pub async fn sparql_get(
    depot: &mut Depot,
    query: QueryParam<String, true>
) -> Result<Json<serde_json::Value>, StatusError> {

    let db = depot.obtain::<Db>().unwrap();


    let result = execute_query(&db.store, &query.into_inner());

    Ok(Json(result))
}