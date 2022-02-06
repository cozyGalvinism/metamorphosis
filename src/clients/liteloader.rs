use std::path::{PathBuf, Path};

use http_cache_reqwest::{Cache, HttpCache, CACacheManager};
use reqwest::Client;
use reqwest_middleware::{ClientWithMiddleware, ClientBuilder};

use crate::models::liteloader::LiteloaderIndex;

pub struct LiteloaderUpdater {
    client: ClientWithMiddleware,
    cache_directory: PathBuf,
}

impl LiteloaderUpdater {
    pub fn new<P>(cache_directory: P) -> Self
    where
        P: AsRef<Path>,
    {
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: http_cache_reqwest::CacheMode::Default,
                manager: CACacheManager {
                    path: "./http_cache".to_string(),
                },
                options: None,
            }))
            .build();
        // ensure the cache path and some subdirectories exist
        std::fs::create_dir_all(cache_directory.as_ref().join("liteloader")).unwrap();

        Self {
            client,
            cache_directory: cache_directory.as_ref().to_path_buf(),
        }
    }

    pub async fn generate_meta_cache(&self) -> std::io::Result<()> {
        info!("Downloading Liteloader index");
        let liteloader_versions = self.client
            .get("https://dl.liteloader.com/versions/versions.json")
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .error_for_status()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .json::<LiteloaderIndex>()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        let versions_file = std::fs::File::create(self.cache_directory.join("liteloader/versions.json"))?;
        serde_json::to_writer_pretty(versions_file, &liteloader_versions)?;

        Ok(())
    }
}