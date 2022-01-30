use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use http_cache_reqwest::{CACacheManager, Cache, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::models::mojang::MojangIndex;

pub struct MojangUpdater {
    client: ClientWithMiddleware,
    upstream_path: PathBuf,
}

impl MojangUpdater {
    pub fn new<P>(upstream_path: P) -> Self
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
        // ensure the upstream path and some subdirectories exist
        std::fs::create_dir_all(upstream_path.as_ref().join("mojang/versions")).unwrap();
        std::fs::create_dir_all(upstream_path.as_ref().join("mojang/assets")).unwrap();

        MojangUpdater {
            client,
            upstream_path: upstream_path.as_ref().to_path_buf(),
        }
    }

    fn get_local_mojang_index(&self) -> MojangIndex {
        info!("Loading local Mojang index...");
        let local_versions: MojangIndex;
        // check if upstream/mojang/version_manifest_v2.json exists,
        // if it does, read it and parse it
        // if it doesn't, create a default MojangIndex
        if let Ok(mut file) =
            std::fs::File::open(self.upstream_path.join("mojang/version_manifest_v2.json"))
        {
            info!("Found local Mojang index!");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            local_versions = serde_json::from_str(&contents).unwrap();
        } else {
            info!("No local Mojang index found, creating empty Mojang index...");
            local_versions = MojangIndex {
                latest: HashMap::new(),
                versions: Vec::new(),
                version_map: RefCell::new(HashMap::new()),
            };
        }

        local_versions
    }

    async fn get_remote_mojang_index(&self) -> std::io::Result<MojangIndex<'_>> {
        info!("Downloading remote Mojang index...");
        // download the mojang index from https://launchermeta.mojang.com/mc/game/version_manifest_v2.json
        // and parse it
        let response = self
            .client
            .get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        info!("Downloaded remote Mojang index!");

        response
            .json()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    /// Updates the Mojang metadata.
    pub async fn generate_meta_cache(&self) -> std::io::Result<()> {
        // Get the local Mojang index
        let local_index = self.get_local_mojang_index();

        // Create a list of version IDs from the list of versions
        let local_version_ids = local_index
            .version_map()
            .keys()
            .cloned()
            .collect::<Vec<String>>();

        // Get the remote Mojang index
        let remote_index = self.get_remote_mojang_index().await?;
        let remote_version_ids = remote_index
            .version_map()
            .keys()
            .cloned()
            .collect::<Vec<String>>();

        // Create a list of versions that are in the remote Mojang index but not in the local Mojang index
        let mut new_versions = remote_version_ids
            .iter()
            .filter(|id| !local_version_ids.contains(id))
            .cloned()
            .collect::<Vec<String>>();
        info!(
            "Found {} new versions, which aren't in the local index!",
            new_versions.len()
        );
        // Create a list of versions that are in the local and remote Mojang index
        let common_versions = local_version_ids
            .iter()
            .filter(|id| remote_version_ids.contains(id))
            .cloned()
            .collect::<Vec<String>>();
        info!(
            "Found {} versions, which are in the local and remote index!",
            common_versions.len()
        );
        info!("Checking if any of the common versions are outdated...");
        for id in common_versions {
            // check if the remote version time is newer than the local version time
            let version_map = remote_index.version_map.borrow();
            let remote_version = version_map.get(&id).unwrap();
            let local_version = version_map.get(&id).unwrap();

            if remote_version.time > local_version.time {
                info!("Version {} is outdated, adding to update list.", id);
                new_versions.push(id);
            }
        }

        let mut asset_map: HashMap<String, String> = HashMap::new();
        for id in new_versions {
            let version_map = remote_index.version_map.borrow();
            let version = version_map.get(&id).unwrap();
            info!("Downloading version file {}...", id);
            let (asset_id, asset_url) = self
                .download_version_file(
                    self.upstream_path
                        .join(format!("mojang/versions/{}.json", id)),
                    &version.url,
                )
                .await?;
            asset_map.insert(asset_id, asset_url);
        }

        for (asset_id, asset_url) in asset_map {
            info!("Downloading asset file {}...", asset_id);
            self.download_asset_file(
                self.upstream_path
                    .join(format!("mojang/assets/{}.json", asset_id)),
                &asset_url,
            )
            .await?;
        }

        info!("Saving new Mojang index...");
        // write the new Mojang index to disk
        let mut file =
            std::fs::File::create(self.upstream_path.join("mojang/version_manifest_v2.json"))?;
        file.write_all(serde_json::to_string(&remote_index).unwrap().as_bytes())?;
        info!("Generation done!");

        Ok(())
    }

    /// Downloads and saves the Mojang version file at the given URL, saves it in the specified path
    /// and returns the asset id and url.
    pub async fn download_version_file<P>(
        &self,
        path: P,
        url: &str,
    ) -> std::io::Result<(String, String)>
    where
        P: AsRef<Path>,
    {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;
        if !response.status().is_success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Downloading version file at {} returned status code {}",
                    url,
                    response.status()
                ),
            ));
        }

        let version_json = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;
        let asset_id = version_json["assetIndex"]["id"].as_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "asset index id not found")
        })?;
        let asset_url = version_json["assetIndex"]["url"].as_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "asset index url not found")
        })?;

        let mut file = std::fs::File::create(path)?;
        file.write_all(
            serde_json::to_string_pretty(&version_json)
                .unwrap()
                .as_bytes(),
        )?;

        Ok((asset_id.to_string(), asset_url.to_string()))
    }

    pub async fn download_asset_file<P>(&self, path: P, url: &str) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;
        if !response.status().is_success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Downloading asset file at {} returned status code {}",
                    url,
                    response.status()
                ),
            ));
        }
        let json = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;

        let mut file = std::fs::File::create(path)?;
        file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes())?;

        Ok(())
    }
}
