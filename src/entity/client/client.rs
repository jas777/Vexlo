use tokio::stream::StreamExt;

use twilight::{
    cache::{
        twilight_cache_inmemory::config::{EventType, InMemoryConfigBuilder},
        InMemoryCache,
    },
    gateway::cluster::{config::ShardScheme, Cluster, ClusterConfig},
    http::Client as HttpClient,
    model::gateway::GatewayIntents,
};

use std::error::Error;
use std::sync::{ Arc };
use std::collections::HashMap;

use crate::entity::handler::EventHandler;
use crate::entity::module::Module;

use crate::modules::core::CoreModule;
use crate::modules::audio::AudioModule;

#[derive(Debug, Clone)]
pub struct Client {
    token: String,
    pub http: HttpClient,
    pub shard_scheme: ShardScheme,
    pub cluster: Cluster,
    pub cache: InMemoryCache,
    pub event_handler: Arc<dyn EventHandler + Send + Sync>,
    pub modules: HashMap<String, Arc<dyn Module + Send + Sync>>
}

impl Client {
    pub async fn new(token: String, event_handler: Arc<dyn EventHandler + Send + Sync>) -> Self {
        let scheme = ShardScheme::Auto;

        let config = ClusterConfig::builder(&token)
            .shard_scheme(scheme)
            .intents(Some(
                GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES,
            ))
            .build();

        let cluster = Cluster::new(config).await;

        let http = HttpClient::new(&token);

        let cache_config = InMemoryConfigBuilder::new()
            .event_types(
                EventType::MESSAGE_CREATE
                    | EventType::MESSAGE_DELETE
                    | EventType::MESSAGE_DELETE_BULK
                    | EventType::MESSAGE_UPDATE,
            )
            .build();

        let cache = InMemoryCache::from(cache_config);

        Client {
            token: token.clone(),
            http: http,
            shard_scheme: scheme,
            cache: cache,
            cluster: cluster.unwrap(),
            event_handler: event_handler,
            modules: {
                let mut list: HashMap<String, Arc<dyn Module + Send + Sync>> = HashMap::new();
    
                list.insert("core".to_string(), Arc::new(CoreModule::default()));
                list.insert("music".to_string(), Arc::new(AudioModule::default()));
    
                list
            }
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        &self.cluster.up().await;

        let events = &mut self.cluster.events().await;
        
        while let Some(event) = events.next().await {
            &self
                .cache
                .update(&event.1)
                .await
                .expect("Cache failed, OhNoe");

                
            tokio::spawn({
                let client_clone: Client = self.clone();
                let handler_clone: Arc<dyn EventHandler + Send + Sync> = self.event_handler.clone();
                async move {
                    handler_clone.handle_event(event, Arc::new(client_clone.clone())).await.expect("Event handler failed");
                }
            });
                
        }

        Ok(())
    }
}
