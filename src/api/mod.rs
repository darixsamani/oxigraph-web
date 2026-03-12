use salvo::prelude::*;
use salvo_oapi::endpoint;
use crate::db::oxigraph::Db;
use crate::services::triple_service::insert_triple;
use salvo_oapi::extract::JsonBody;

use crate::schemas::triples::TripleInput;


#[endpoint(
    tags("Oxigraph"),
    summary = "This endpoint allows you to add a triple to the Oxigraph store",
    description = "This endpoint accepts a JSON body with the subject, predicate, and object of the triple you want to add. It then inserts the triple into the Oxigraph store and returns a success message if the operation was successful."
)]
pub async fn add_triple(depot: &mut Depot, data: JsonBody<TripleInput>) -> Result<String, StatusError> {
    
    let db = depot.obtain::<Db>().unwrap();

    insert_triple(&db.store, data.into_inner())
    .map_err(|_ | StatusError::internal_server_error())?;
    
    Ok("Triple added successfully".to_string())
}