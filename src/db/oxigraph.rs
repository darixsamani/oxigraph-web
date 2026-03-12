use oxigraph::store::Store;
use std::sync::Arc;

#[derive(Clone)]
pub struct  Db {
    pub store: Arc<Store>,
}

impl std::fmt::Debug for Db {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Db")
            .field("store", &"<Store omitted>")
            .finish()
    }
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