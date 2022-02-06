use std::{
    collections::HashMap,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use http_cache_reqwest::{CACacheManager, Cache, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use crate::models::{
    forge::{
        DerivedForgeIndex, ForgeEntry, ForgeFile, ForgeInstallerProfile, ForgeInstallerProfileV1_5,
        ForgeInstallerProfileV2, ForgeMCVersionInfo, ForgeVersion, InstallerInfo,
    },
    mojang::MojangVersionFile,
};

static FORGE_LEGACY_INFO: &str = include_str!("static_files/forge_legacyinfo.json");

lazy_static! {
    static ref PROMOTED_KEY_REGEX: regex::Regex = regex::Regex::new("(?P<mc>[^-]+)-(?P<promotion>(latest)|(recommended))(-(?P<branch>[a-zA-Z0-9\\.]+))?").unwrap();
    static ref VERSION_REGEX: regex::Regex = regex::Regex::new("^(?P<mc>[0-9a-zA-Z_\\.]+)-(?P<ver>[0-9\\.]+\\.(?P<build>[0-9]+))(-(?P<branch>[a-zA-Z0-9\\.]+))?$").unwrap();
}

pub struct ForgeUpdater {
    client: ClientWithMiddleware,
    cache_directory: PathBuf,
}

impl ForgeUpdater {
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
        std::fs::create_dir_all(cache_directory.as_ref().join("forge/jars")).unwrap();
        std::fs::create_dir_all(cache_directory.as_ref().join("forge/installer_info")).unwrap();
        std::fs::create_dir_all(cache_directory.as_ref().join("forge/installer_manifests"))
            .unwrap();
        std::fs::create_dir_all(cache_directory.as_ref().join("forge/version_manifests")).unwrap();
        std::fs::create_dir_all(cache_directory.as_ref().join("forge/files_manifests")).unwrap();

        Self {
            client,
            cache_directory: cache_directory.as_ref().to_path_buf(),
        }
    }

    pub async fn generate_meta_cache(&self) -> std::io::Result<()> {
        info!("Downloading remote version list from Forge...");
        let remote_list = self
            .client
            .get("https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json")
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;

        info!("Downloading promotion list from Forge...");
        let promotions_list = self
            .client
            .get("https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json")
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;
        let mut new_index = DerivedForgeIndex {
            mc_versions: Some(HashMap::new()),
            versions: Some(HashMap::new()),
        };

        let mut recommended: Vec<String> = Vec::new();
        let promos = promotions_list
            .as_object()
            .unwrap()
            .get("promos")
            .unwrap()
            .as_object()
            .unwrap();
        info!("Processing promotions...");
        for (promo_key, short_version) in promos {
            let key_match = PROMOTED_KEY_REGEX.captures(promo_key);
            if key_match.is_none() {
                info!(
                    "Skipping promo key {}, key was not in the right format",
                    promo_key
                );
                continue;
            }
            let key_match = key_match.unwrap();
            if key_match.name("mc").is_none() {
                info!(
                    "Skipping promo key {}, it has no Minecraft version",
                    promo_key
                );
                continue;
            }
            if key_match.name("branch").is_some() {
                info!("Skipping promo key {}, it has a branch", promo_key);
                continue;
            } else if key_match.name("promotion").unwrap().as_str() == "recommended" {
                info!(
                    "Adding recommendation for version {}",
                    short_version.as_str().unwrap()
                );
                recommended.push(short_version.as_str().unwrap().to_string());
            } else if key_match.name("promotion").unwrap().as_str() == "latest" {
                continue;
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unknown promotion type: {}", promo_key),
                ));
            }
        }

        for (mc_version, value) in remote_list.as_object().unwrap() {
            if !value.is_array() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Invalid metadata format while processing version {} (MC version value was not an array)", mc_version),
                ));
            }
            let value = value.as_array().unwrap();
            for long_version in value {
                if !long_version.is_string() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Invalid metadata format while processing version {} (Forge version is not a string)", mc_version),
                    ));
                }
                let long_version = long_version.as_str().unwrap();
                let version_match = VERSION_REGEX.captures(long_version);
                if version_match.is_none() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Invalid metadata while processing version {} (Version doesn't match regex)", mc_version),
                    ));
                }
                let version_match = version_match.unwrap();
                let mc_group = version_match.name("mc").unwrap();
                if mc_group.as_str() != mc_version {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Invalid metadata while processing version {} (MC version doesn't match)", mc_version),
                    ));
                }
                info!(
                    "Downloading manifest for MC version {}, Forge version {}",
                    mc_version, long_version
                );
                let files = self
                    .download_single_forge_file_manifest(long_version)
                    .await?;
                let build = version_match
                    .name("build")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let version = version_match.name("ver").unwrap().as_str();
                let branch = version_match.name("branch").map(|x| x.as_str().to_string());

                let is_recommended = recommended.contains(&version.to_string());

                let entry = ForgeEntry {
                    long_version: long_version.to_string(),
                    mc_version: mc_version.to_string(),
                    build,
                    version: version.to_string(),
                    branch,
                    latest: Some(false),
                    recommended: Some(is_recommended),
                    files: Some(files),
                };

                new_index
                    .versions
                    .as_mut()
                    .unwrap()
                    .insert(long_version.to_string(), entry.clone());
                if !new_index
                    .mc_versions
                    .as_ref()
                    .unwrap()
                    .contains_key(mc_version)
                {
                    new_index.mc_versions.as_mut().unwrap().insert(
                        mc_version.to_string(),
                        ForgeMCVersionInfo {
                            latest: None,
                            recommended: None,
                            versions: Some(Vec::new()),
                        },
                    );
                }
                new_index
                    .mc_versions
                    .as_mut()
                    .unwrap()
                    .get_mut(mc_version)
                    .unwrap()
                    .versions
                    .as_mut()
                    .unwrap()
                    .push(long_version.to_string());
                if let Some(recommended) = entry.recommended {
                    if recommended {
                        new_index
                            .mc_versions
                            .as_mut()
                            .unwrap()
                            .get_mut(mc_version)
                            .unwrap()
                            .recommended
                            .replace(long_version.to_string());
                    }
                }
            }
        }

        info!("Post-processing promotions...");
        for (mc_version, mut info) in new_index.mc_versions.as_mut().unwrap() {
            let latest_version = info.versions.as_ref().unwrap().iter().last().unwrap();
            info.latest = Some(latest_version.clone());
            new_index
                .versions
                .as_mut()
                .unwrap()
                .get_mut(latest_version)
                .unwrap()
                .latest = Some(true);
            info!(
                "Added {} as latest version for MC version {}",
                latest_version, mc_version
            );
        }

        info!("Dumping index files...");
        let maven_file =
            std::fs::File::create(self.cache_directory.join("forge/maven-metadata.json"))?;
        serde_json::to_writer_pretty(maven_file, &remote_list)?;

        let promotions_file =
            std::fs::File::create(self.cache_directory.join("forge/promotion_slim.json"))?;
        serde_json::to_writer_pretty(promotions_file, &promotions_list)?;

        let index_file =
            std::fs::File::create(self.cache_directory.join("forge/derived_index.json"))?;
        serde_json::to_writer_pretty(index_file, &new_index)?;

        info!("Downloading installers and dumping profiles...");
        for (id, entry) in new_index.versions.as_ref().unwrap() {
            let version: ForgeVersion = entry.clone().into();
            if version.url().is_none() {
                info!("Skipping build {}: No valid files", entry.build);
                continue;
            }

            let jar_file_path = self
                .cache_directory
                .join(format!("forge/jars/{}", version.file_name().unwrap()));

            if version.uses_installer() {
                let installer_info_file_path = self.cache_directory.join(format!(
                    "forge/installer_info/{}.json",
                    version.long_version
                ));
                let profile_file_path = self.cache_directory.join(format!(
                    "forge/installer_manifests/{}.json",
                    version.long_version
                ));
                let version_json_file_path = self.cache_directory.join(format!(
                    "forge/version_manifests/{}.json",
                    version.long_version
                ));

                let mut installer_refresh_required = false;
                if !profile_file_path.is_file() {
                    installer_refresh_required = true;
                }
                if !installer_info_file_path.is_file() {
                    installer_refresh_required = true;
                }

                if installer_refresh_required && !jar_file_path.is_file() {
                    info!("Downloading Forge version {}...", version.long_version);
                    let version_installer = self
                        .client
                        .get(version.url().unwrap())
                        .send()
                        .await
                        .map_err(|e| {
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!(
                                    "Failed to download installer for version {}: {}",
                                    version.long_version, e
                                ),
                            )
                        })?;
                    if !version_installer.status().is_success() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!(
                                "Failed to download installer for version {}: {}",
                                version.long_version,
                                version_installer.status()
                            ),
                        ));
                    }
                    let mut installer_file = std::fs::File::create(&jar_file_path)?;
                    let version_installer = version_installer.bytes().await.map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!(
                                "Failed to download installer for version {}: {}",
                                version.long_version, e
                            ),
                        )
                    })?;
                    installer_file.write_all(&version_installer)?;
                }

                info!(
                    "Processing installer for version {}...",
                    version.long_version
                );
                if !profile_file_path.is_file() {
                    // read jar_file_path as zip
                    let mut zip = zip::ZipArchive::new(std::fs::File::open(&jar_file_path)?)?;
                    // read version info
                    if let Ok(version_json_entry) = zip.by_name("version.json") {
                        let version_json_data: serde_json::Result<MojangVersionFile> =
                            serde_json::from_reader(version_json_entry);
                        if version_json_data.is_err() {
                            warn!(
                                "Failed to parse version.json for version {}",
                                version.long_version
                            );
                        } else {
                            let version_json_data = version_json_data.unwrap();
                            let mut version_json_file =
                                std::fs::File::create(&version_json_file_path)?;
                            serde_json::to_writer_pretty(
                                &mut version_json_file,
                                &version_json_data,
                            )?;
                        }
                    }

                    // read install profile
                    {
                        let mut install_profile_entry = zip.by_name("install_profile.json")?;

                        let mut install_profile_data_str = String::new();
                        install_profile_entry.read_to_string(&mut install_profile_data_str)?;
                        // check if data can be parsed to either ForgeInstallerProfile, ForgeInstallerProfileV2 or ForgeInstallerProfileV1_5
                        let install_profile_data: serde_json::Result<ForgeInstallerProfile> =
                            serde_json::from_str(&install_profile_data_str);
                        let install_profile_data_v2: serde_json::Result<ForgeInstallerProfileV2> =
                            serde_json::from_str(&install_profile_data_str);
                        let install_profile_data_v1_5: serde_json::Result<
                            ForgeInstallerProfileV1_5,
                        > = serde_json::from_str(&install_profile_data_str);

                        if install_profile_data.is_ok() {
                            let install_profile_data = install_profile_data.unwrap();
                            let mut install_profile_file =
                                std::fs::File::create(&profile_file_path)?;
                            serde_json::to_writer_pretty(
                                &mut install_profile_file,
                                &install_profile_data,
                            )?;
                        } else if install_profile_data_v2.is_ok() {
                            let install_profile_data_v2 = install_profile_data_v2.unwrap();
                            let mut install_profile_file =
                                std::fs::File::create(&profile_file_path)?;
                            serde_json::to_writer_pretty(
                                &mut install_profile_file,
                                &install_profile_data_v2,
                            )?;
                        } else if install_profile_data_v1_5.is_ok() {
                            let install_profile_data_v1_5 = install_profile_data_v1_5.unwrap();
                            let mut install_profile_file =
                                std::fs::File::create(&profile_file_path)?;
                            serde_json::to_writer_pretty(
                                &mut install_profile_file,
                                &install_profile_data_v1_5,
                            )?;
                        } else if version.is_supported() {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!(
                                    "Failed to parse install_profile.json for version {}",
                                    version.long_version
                                ),
                            ));
                        } else {
                            warn!(
                                "Failed to parse install_profile.json for version {}",
                                version.long_version
                            );
                        }
                    }
                }

                if !installer_info_file_path.is_file() {
                    // sha1 of the file at jar_file_path using ring
                    let sha1_hash = ring::digest::digest(
                        &ring::digest::SHA1_FOR_LEGACY_USE_ONLY,
                        &std::fs::read(&jar_file_path)?,
                    );
                    let sha1 = data_encoding::HEXLOWER.encode(sha1_hash.as_ref());
                    // sha256 of the file at jar_file_path using ring
                    let sha256_hash = ring::digest::digest(
                        &ring::digest::SHA256,
                        &std::fs::read(&jar_file_path)?,
                    );
                    let sha256 = data_encoding::HEXLOWER.encode(sha256_hash.as_ref());
                    // size of the file at jar_file_path
                    let size = std::fs::metadata(&jar_file_path)?.len();
                    let installer_info = InstallerInfo {
                        sha1_hash: Some(sha1),
                        sha256_hash: Some(sha256),
                        size: Some(size),
                    };
                    let mut installer_info_file = std::fs::File::create(&installer_info_file_path)?;
                    serde_json::to_writer_pretty(&mut installer_info_file, &installer_info)?;
                }
            }
        }

        // write static legacy info if it doesn't exist
        if !PathBuf::new()
            .join("static/forge-legacyinfo.json")
            .is_file()
        {
            let mut forge_legacyinfo_file =
                std::fs::File::create(&PathBuf::new().join("static/forge-legacyinfo.json"))?;
            let _ = forge_legacyinfo_file.write(FORGE_LEGACY_INFO.as_bytes())?;
        }

        Ok(())
    }

    pub async fn download_single_forge_file_manifest(
        &self,
        long_version: &str,
    ) -> std::io::Result<HashMap<String, ForgeFile>> {
        let manifest_path = self
            .cache_directory
            .join(format!("forge/files_manifests/{}.json", long_version));
        let mut from_file = false;
        let files_json: serde_json::Value;
        if manifest_path.is_file() {
            files_json =
                serde_json::from_reader(std::fs::File::open(&manifest_path).unwrap()).unwrap();
            from_file = true;
            info!("Using cached file manifest for version {}", long_version);
        } else {
            files_json = self
                .client
                .get(format!(
                    "https://files.minecraftforge.net/net/minecraftforge/forge/{}/meta.json",
                    long_version
                ))
                .send()
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?
                .json::<serde_json::Value>()
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))?;
        }

        let mut file_map: HashMap<String, ForgeFile> = HashMap::new();
        for (classifier, extension_obj) in files_json
            .as_object()
            .unwrap()
            .get("classifiers")
            .unwrap()
            .as_object()
            .unwrap()
        {
            let extension_obj = extension_obj.as_object().expect("a json object");
            let mut index = 0;
            let mut inserted = false;
            while index < extension_obj.len() {
                let (extension, hash) = extension_obj.iter().last().unwrap();
                if !hash.is_string() {
                    warn!("{}: Skipping missing hash for {}", long_version, extension);
                    info!("{}", serde_json::to_string_pretty(&extension_obj).unwrap());
                    index += 1;
                    continue;
                }
                let hash = hash.as_str().unwrap();

                let processing_regex = regex::Regex::new(r"\W").unwrap();
                let processed_hash = processing_regex.replacen(hash, 1, "");
                if processed_hash.len() != 32 {
                    warn!("{}: Skipping invalid hash for {}", long_version, extension);
                    index += 1;
                    continue;
                }

                let file = ForgeFile {
                    classifier: classifier.to_string(),
                    hash: processed_hash.to_string(),
                    extension: extension.to_string(),
                };
                if !inserted {
                    file_map.insert(classifier.to_string(), file);
                    index += 1;
                    inserted = true;
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{}: Duplicate classifier {}", long_version, classifier),
                    ));
                }
            }
        }

        if !from_file {
            std::fs::write(manifest_path, serde_json::to_string_pretty(&files_json)?)?;
        }

        Ok(file_map)
    }
}
