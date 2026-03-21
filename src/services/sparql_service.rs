use oxigraph::sparql::{QueryResults, UpdateEvaluationError};
use oxigraph::store::Store;
use serde_json::json;


#[allow(deprecated)]
pub fn execute_query(store: &Store, query: &str ) -> serde_json::Value {
    match store.query(query) {

        Ok(QueryResults::Solutions(solutions)) =>{

            let mut rows = Vec::new();
            for solution in solutions {
                let solution = solution.unwrap();
                let mut row = serde_json::Map::new();
                for (var, value) in solution.iter() {
                    row.insert(
                        var.to_string(),
                        json!(value.to_string())
                    );
                rows.push(json!(row));
                }       
        }
            json!(rows)
        },

        Ok(QueryResults::Boolean(result)) => {
            json!({"boolean": result})
        },

        _ => json!({"error": "Query execution failed"})
    }
}

pub fn execute_update(store: &Store, query: &str) -> Result<(), UpdateEvaluationError> {
    store.update(query)
}