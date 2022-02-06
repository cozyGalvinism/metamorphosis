use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use chrono::DateTime;
use http_cache_reqwest::{CACacheManager, Cache, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::models::fabric::FabricJarInfo;

fn get_maven_url(maven_key: &str, server: &str, ext: &str) -> String {
    let maven_parts = maven_key.splitn(3, ':').collect::<Vec<&str>>();
    let maven_ver_url = format!(
        "{}{}/{}/{}/",
        server,
        maven_parts[0].replace('.', "/"),
        maven_parts[1],
        maven_parts[2]
    );
    let maven_url = format!(
        "{}{}-{}{}",
        maven_ver_url, maven_parts[1], maven_parts[2], ext
    );
    maven_url
}

pub struct FabricUpdater {
    client: ClientWithMiddleware,
    cache_directory: PathBuf,
}

impl FabricUpdater {
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
        std::fs::create_dir_all(cache_directory.as_ref().join("fabric/meta-v2")).unwrap();
        std::fs::create_dir_all(
            cache_directory
                .as_ref()
                .join("fabric/loader-installer-json"),
        )
        .unwrap();
        std::fs::create_dir_all(cache_directory.as_ref().join("fabric/jars")).unwrap();

        Self {
            client,
            cache_directory: cache_directory.as_ref().to_path_buf(),
        }
    }

    async fn download_json_file<P>(&self, path: P, url: &str) -> std::io::Result<serde_json::Value>
    where
        P: AsRef<Path>,
    {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .error_for_status()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(&mut file, &response)?;
        Ok(response)
    }

    async fn download_binary_file<P>(&self, path: P, url: &str) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .error_for_status()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let mut file = std::fs::File::create(path)?;
        // write response.bytes() to file
        let bytes = response
            .bytes()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        file.write_all(&bytes)?;

        Ok(())
    }

    async fn process_jar_file<P>(&self, path: P, url: &str) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let jar_path = format!("{}.jar", path.as_ref().to_str().unwrap());
        self.download_binary_file(&jar_path, url).await?;
        let mut timestamp =
            chrono::DateTime::from_utc(chrono::NaiveDateTime::from_timestamp(0, 0), chrono::Utc);
        let mut jar_file = zip::ZipArchive::new(std::fs::File::open(&jar_path)?)?;
        for i in 0..jar_file.len() {
            let mut file = jar_file.by_index(i)?;
            let file_last_modified = file.last_modified();
            let file_last_modified = chrono::DateTime::<chrono::Utc>::from_utc(
                chrono::NaiveDateTime::new(
                    chrono::NaiveDate::from_ymd(
                        file_last_modified.year().into(),
                        file_last_modified.month().into(),
                        file_last_modified.day().into(),
                    ),
                    chrono::NaiveTime::from_hms(
                        file_last_modified.hour().into(),
                        file_last_modified.minute().into(),
                        file_last_modified.second().into(),
                    ),
                ),
                chrono::Utc,
            );
            if file_last_modified > timestamp {
                timestamp = file_last_modified;
            }
        }

        let sha1_hash = ring::digest::digest(
            &ring::digest::SHA1_FOR_LEGACY_USE_ONLY,
            &std::fs::read(&jar_path)?,
        );
        let sha1 = data_encoding::HEXLOWER.encode(sha1_hash.as_ref());
        let sha256_hash = ring::digest::digest(&ring::digest::SHA256, &std::fs::read(&jar_path)?);
        let sha256 = data_encoding::HEXLOWER.encode(sha256_hash.as_ref());
        let size = std::fs::metadata(&jar_path)?.len();

        let data = FabricJarInfo {
            release_time: Some(timestamp),
            sha1: Some(sha1),
            sha256: Some(sha256),
            size: Some(size),
        };
        let mut file = std::fs::File::create(format!("{}.json", path.as_ref().to_str().unwrap()))?;
        serde_json::to_writer_pretty(&mut file, &data)?;

        Ok(())
    }

    pub async fn generate_meta_cache(&self) -> std::io::Result<()> {
        for component in &["intermediary", "loader"] {
            info!("Downloading JSON for {} meta...", component);
            let index = self
                .download_json_file(
                    self.cache_directory
                        .join(format!("fabric/meta-v2/{}.json", component)),
                    &format!("https://meta.fabricmc.net/v2/versions/{}", component),
                )
                .await?;
            for it_value in index.as_array().unwrap() {
                let it_value = it_value.as_object().unwrap();
                let it_maven = it_value.get("maven").unwrap().as_str().unwrap();
                info!("Downloading jar for artifact {}...", it_maven);
                let jar_maven_url = get_maven_url(it_maven, "https://maven.fabricmc.net/", ".jar");
                self.process_jar_file(
                    self.cache_directory
                        .join(format!("fabric/jars/{}", it_maven.replace(':', "."))),
                    &jar_maven_url,
                )
                .await?;
            }
        }

        let loader_json =
            std::fs::File::open(self.cache_directory.join("fabric/meta-v2/loader.json"))?;
        let loader_version_index: serde_json::Value = serde_json::from_reader(loader_json)?;
        let loader_version_index = loader_version_index.as_array().unwrap();
        for it_value in loader_version_index {
            let it_value = it_value.as_object().unwrap();
            let it_maven = it_value.get("maven").unwrap().as_str().unwrap();
            let maven_url = get_maven_url(
                it_maven,
                "https://maven.fabricmc.net/",
                ".json",
            );
            info!("Downloading installer JSON for artifact {} from {}...", it_maven, &maven_url);
            self.download_json_file(
                self.cache_directory.join(format!(
                    "fabric/loader-installer-json/{}.json",
                    it_value.get("version").unwrap().as_str().unwrap()
                )),
                &maven_url,
            )
            .await?;
        }

        Ok(())
    }
}
