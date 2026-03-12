use oxigraph::store::Store;
use std::sync::Arc;

#[derive(Clone)]
pub struct  Db {
    pub store: Arc<Store>,
}

impl Db {
    pub fn new() -> Self{
        
        let store = Store::open("data")
                    .expect("Failed to open persistent store");
        Self {
            store: Arc::new(store),
        }
    }
}