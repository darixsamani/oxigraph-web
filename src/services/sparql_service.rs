use oxigraph::sparql::{QueryResults, UpdateEvaluationError, SparqlEvaluator};
use oxigraph::store::Store;
use serde_json::json;


pub fn execute_query(store: &Store, query: &str) -> serde_json::Value {
    let parse_result = SparqlEvaluator::new().parse_query(query);

    match parse_result {
        Ok(query_obj) => {
            match query_obj.on_store(store).execute() {
                Ok(QueryResults::Solutions(solutions)) => {
                    let mut rows = Vec::new();
                    for solution in solutions {
                        let solution = solution.unwrap();
                        let mut row = serde_json::Map::new();
                        for (var, value) in solution.iter() {
                            row.insert(var.to_string(), json!(value.to_string()));
                        }
                        rows.push(json!(row));
                    }
                    json!(rows)
                }
                Ok(QueryResults::Boolean(result)) => {
                    json!({"boolean": result})
                }
                Ok(_) => json!({"error": "Unsupported query result type"}),
                Err(e) => json!({"error": format!("Query execution failed: {}", e)}),
            }
        }
        Err(e) => json!({"error": format!("Query parsing failed: {}", e)}),
    }
}

pub fn execute_update(store: &Store, query: &str) -> Result<(), UpdateEvaluationError> {
    store.update(query)
}