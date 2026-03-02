pub mod models;
pub mod queries;

use mongodb::{Client, Database as MongoDatabase};

pub struct Database {
    pub queries: queries::Queries,
}

impl Database {
    pub async fn new(uri: &str, database_name: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(database_name);
        
        Ok(Self {
            queries: queries::Queries::new(&db),
        })
    }
}
