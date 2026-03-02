pub mod client;
pub mod data;

pub use data::BotData;

use std::sync::Arc;
use tracing::{info, error};

pub async fn start_bot(data: Arc<BotData>) {
    info!("Initializing Discord bot client...");
    
    match client::create_client(data).await {
        Ok(mut client) => {
            if let Err(e) = client.start().await {
                error!("Bot client error: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to create bot client: {}", e);
        }
    }
}
