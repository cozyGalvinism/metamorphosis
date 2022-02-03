use std::str::FromStr;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use super::{
    misc::GradleSpecifier,
    mojang::{
        MojangArtifact, MojangArtifactBase, MojangAssets, MojangError, MojangLibrary,
        MojangLibraryDownloads, MojangVersionFile,
    },
};

lazy_static! {
    pub static ref CURRENT_POLYMC_FORMAT_VERSION: u8 = 1;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PolyMCLibrary {
    #[serde(flatten)]
    pub library: MojangLibrary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "MMC-hint", skip_serializing_if = "Option::is_none")]
    mmc_hint: Option<String>,
}

impl From<MojangLibrary> for PolyMCLibrary {
    fn from(lib: MojangLibrary) -> Self {
        Self {
            url: None,
            mmc_hint: None,
            library: lib,
        }
    }
}

fn default_format_version() -> u8 {
    *CURRENT_POLYMC_FORMAT_VERSION
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionedJsonObject {
    #[serde(
        rename = "formatVersion",
        default = "default_format_version",
        with = "crate::validators::polymc_version_validation"
    )]
    pub format_version: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DependencyEntry {
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggests: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PolyMCVersionFile {
    #[serde(flatten)]
    pub versioned_json_object: VersionedJsonObject,
    pub name: String,
    pub version: String,
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<DependencyEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<DependencyEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatile: Option<bool>,
    #[serde(rename = "assetIndex", skip_serializing_if = "Option::is_none")]
    pub asset_index: Option<MojangAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<PolyMCLibrary>>,
    #[serde(rename = "mavenFiles", skip_serializing_if = "Option::is_none")]
    pub maven_files: Option<Vec<PolyMCLibrary>>,
    #[serde(rename = "mainJar", skip_serializing_if = "Option::is_none")]
    pub main_jar: Option<PolyMCLibrary>,
    #[serde(rename = "jarMods", skip_serializing_if = "Option::is_none")]
    pub jar_mods: Option<Vec<PolyMCLibrary>>,
    #[serde(rename = "mainClass", skip_serializing_if = "Option::is_none")]
    pub main_class: Option<String>,
    #[serde(rename = "appletClass", skip_serializing_if = "Option::is_none")]
    pub applet_class: Option<String>,
    #[serde(rename = "minecraftArguments", skip_serializing_if = "Option::is_none")]
    pub minecraft_arguments: Option<String>,
    #[serde(rename = "releaseTime", skip_serializing_if = "Option::is_none")]
    pub release_time: Option<DateTime<chrono::Utc>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub version_file_type: Option<String>,
    #[serde(rename = "+traits", skip_serializing_if = "Option::is_none")]
    pub add_traits: Option<Vec<String>>,
    #[serde(rename = "+tweakers", skip_serializing_if = "Option::is_none")]
    pub add_tweakers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

impl PolyMCVersionFile {
    pub fn new(name: String, version: String, uid: String) -> Self {
        Self {
            versioned_json_object: VersionedJsonObject {
                format_version: *CURRENT_POLYMC_FORMAT_VERSION,
            },
            name,
            version,
            uid,
            requires: None,
            conflicts: None,
            volatile: None,
            asset_index: None,
            libraries: None,
            maven_files: None,
            main_jar: None,
            jar_mods: None,
            main_class: None,
            applet_class: None,
            minecraft_arguments: None,
            release_time: None,
            version_file_type: None,
            add_traits: None,
            add_tweakers: None,
            order: None,
        }
    }

    /// Converts a MojangVersionFile to a PolyMCVersionFile
    pub fn from_mojang_file(
        file: &MojangVersionFile,
        name: String,
        uid: String,
        version: String,
    ) -> Result<Self, MojangError> {
        let mut pmc_file = Self::new(name, version, uid);

        pmc_file.asset_index = file.asset_index.clone();
        // convert Mojang libraries to PolyMC libraries by extending Mojang libraries with Nones
        pmc_file.libraries = file.libraries.as_ref().map(|libraries| {
            libraries
                .iter()
                .map(|lib| PolyMCLibrary::from(lib.clone()))
                .collect()
        });
        pmc_file.main_class = file.main_class.clone();
        if let Some(file_id) = &file.id {
            let mut main_jar = PolyMCLibrary {
                library: MojangLibrary {
                    name: GradleSpecifier::from_str(&format!(
                        "com.mojang:minecraft:{}:client",
                        file_id
                    ))
                    .unwrap(),
                    extract: None,
                    downloads: None,
                    natives: None,
                    rules: None,
                },
                url: None,
                mmc_hint: None,
            };
            let client_downloads = file
                .downloads
                .as_ref()
                .unwrap()
                .get("client")
                .expect("client downloads");
            main_jar.library.downloads = Some(MojangLibraryDownloads {
                artifact: Some(MojangArtifact {
                    artifact_base: MojangArtifactBase {
                        sha1: client_downloads.sha1.clone(),
                        size: client_downloads.size,
                        url: client_downloads.url.clone(),
                    },
                    path: None,
                }),
                classifiers: None,
            });
            pmc_file.main_jar = Some(main_jar);
        }

        pmc_file.minecraft_arguments = file.minecraft_arguments.clone();
        pmc_file.release_time = file.release_time;
        pmc_file.version_file_type = file.version_type.clone();
        let max_supported_compliance_level = 1;
        if let Some(compliance_level) = file.compliance_level {
            if compliance_level == max_supported_compliance_level {
                if pmc_file.add_traits.is_none() {
                    pmc_file.add_traits = Some(Vec::new());
                }

                pmc_file
                    .add_traits
                    .as_mut()
                    .unwrap()
                    .push("XR:Initial".to_string());
            } else {
                return Err(MojangError::UnknownComplicanceLevel {
                    compliance_level,
                    max_supported: max_supported_compliance_level,
                });
            }
        }

        Ok(pmc_file)
    }

    pub fn apply_legacy_override(&mut self, legacy_override: &LegacyOverrideEntry) {
        self.main_class = legacy_override.main_class.clone();
        self.applet_class = legacy_override.applet_class.clone();
        if let Some(release_time) = &legacy_override.release_time {
            self.release_time = Some(*release_time);
        }

        if let Some(add_traits) = &legacy_override.add_traits {
            if self.add_traits.is_none() {
                self.add_traits = Some(Vec::new());
            }

            self.add_traits.as_mut().unwrap().extend(add_traits.clone());
        }

        self.libraries = None;
        self.minecraft_arguments = None;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolyMCSharedPackageData {
    #[serde(flatten)]
    pub versioned_json_object: VersionedJsonObject,
    pub name: String,
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "projectUrl", skip_serializing_if = "Option::is_none")]
    pub project_url: Option<String>,
}

impl PolyMCSharedPackageData {
    /// Writes the package data to `polymc/{uid}/package.json`
    pub fn write(&self) -> std::io::Result<()> {
        let self_serialized = serde_json::to_string(&self)?;

        std::fs::write(format!("polymc/{}/package.json", self.uid), self_serialized)
    }

    /// Creates a new PolyMCSharedPackageData and writes it to `polymc/{uid}/package.json`
    pub fn write_new(uid: String, name: String) -> std::io::Result<()> {
        let pmc_shared_package_data = Self {
            versioned_json_object: VersionedJsonObject {
                format_version: *CURRENT_POLYMC_FORMAT_VERSION,
            },
            name,
            uid,
            recommended: None,
            authors: None,
            description: None,
            project_url: None,
        };
        pmc_shared_package_data.write()
    }

    /// Reads the package data from `polymc/{uid}/package.json`
    pub fn read(uid: String) -> std::io::Result<Self> {
        let file_content = std::fs::read_to_string(format!("polymc/{}/package.json", uid))?;
        let pmc_shared_package_data: Self = serde_json::from_str(&file_content)?;
        Ok(pmc_shared_package_data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolyMCVersionIndexEntry {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_type: Option<String>,
    #[serde(rename = "releaseTime", skip_serializing_if = "Option::is_none")]
    pub release_time: Option<DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<DependencyEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<DependencyEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatile: Option<bool>,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolyMCVersionIndex {
    #[serde(flatten)]
    pub versioned_json_object: VersionedJsonObject,
    pub name: String,
    pub uid: String,
    pub versions: Vec<PolyMCVersionIndexEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolyMCPackageIndexEntry {
    pub name: String,
    pub uid: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolyMCPackageIndex {
    #[serde(flatten)]
    pub versioned_json_object: VersionedJsonObject,
    pub packages: Vec<PolyMCPackageIndexEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyOverrideEntry {
    #[serde(rename = "releaseTime", skip_serializing_if = "Option::is_none")]
    pub release_time: Option<DateTime<chrono::Utc>>,
    #[serde(rename = "mainClass", skip_serializing_if = "Option::is_none")]
    pub main_class: Option<String>,
    #[serde(rename = "appletClass", skip_serializing_if = "Option::is_none")]
    pub applet_class: Option<String>,
    #[serde(rename = "+traits", skip_serializing_if = "Option::is_none")]
    pub add_traits: Option<Vec<String>>,
}
