use salvo::prelude::*;
use salvo_oapi::endpoint;
use crate::schemas::triples::SparqlUpdate;
use crate::services::sparql_service::execute_update;
use crate::db::oxigraph::Db;
use salvo_oapi::extract::JsonBody;

#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to execute a SPARQL update query on the Oxigraph store",
    description = "This endpoint accepts a SPARQL update query in the request body and executes it on the Oxigraph store. It returns a success message if the update was executed successfully, or an error message if the update failed."
)]
pub async fn sparql_update_endpoint(
    depot: &mut Depot,
    data: JsonBody<SparqlUpdate>,
) -> Result<String, StatusError> {
    let db = depot.obtain::<Db>().unwrap();
    execute_update(&db.store, &data.into_inner().query)
        .map_err(|_| StatusError::internal_server_error())?;

    Ok("SPARQL UPDATE executed successfully".into())
}