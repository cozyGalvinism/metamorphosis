use std::{cell::RefCell, collections::HashMap, ops::DerefMut};

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use super::misc::GradleSpecifier;

lazy_static! {
    pub static ref MAX_MOJANG_SUPPORTED_VERSION: i32 = 21;
}

// TODO: Change the supported version if it changes!
custom_error! {
    /// Errors that can occur when parsing a Mojang's version index.
    pub MojangError
        UnknownComplicanceLevel { compliance_level: i32, max_supported: i32 } = "Unsupported Mojang compliance level: {compliance_level}. Max supported is: {max_supported}"
}

/// A single entry of Mojang's version index.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangIndexEntry {
    pub id: String,
    #[serde(rename = "releaseTime")]
    pub release_time: DateTime<chrono::Utc>,
    pub time: DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(rename = "complianceLevel", skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<i32>,
}

/// Mojang's index of all versions.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangIndex<'a> {
    pub latest: HashMap<String, String>,
    pub versions: Vec<MojangIndexEntry>,
    // this has to be a RefCell for interior mutability
    #[serde(skip)]
    pub version_map: RefCell<HashMap<String, &'a MojangIndexEntry>>,
}

impl<'a> MojangIndex<'a> {
    /// Fills the version_map field of the MojangIndex struct (if empty) and returns it.
    pub fn version_map(&'a self) -> impl DerefMut<Target = HashMap<String, &'a MojangIndexEntry>> {
        let mut version_map = self.version_map.borrow_mut();
        if version_map.is_empty() {
            for version in self.versions.iter() {
                version_map.insert(version.id.clone(), version);
            }
        }

        version_map
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangArtifactBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangArtifact {
    #[serde(flatten)]
    pub artifact_base: MojangArtifactBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangAssets {
    #[serde(flatten)]
    pub artifact: MojangArtifactBase,
    id: String,
    #[serde(rename = "totalSize")]
    total_size: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangLibraryDownloads {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<MojangArtifact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<HashMap<String, MojangArtifact>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangLibraryExtractRules {
    pub exclude: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OSName {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "osx")]
    MacOS,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OSRule {
    pub name: OSName,
    pub rules: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MojangAction {
    Allow,
    Disallow,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangRule {
    pub action: MojangAction,
    pub os: Option<OSRule>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangLibrary {
    pub extract: Option<MojangLibraryExtractRules>,
    pub name: GradleSpecifier,
    pub downloads: Option<MojangLibraryDownloads>,
    pub natives: Option<HashMap<String, String>>,
    pub rules: Option<Vec<MojangRule>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangLoggingArtifact {
    #[serde(flatten)]
    pub artifact: MojangArtifactBase,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MojangLoggingType {
    #[serde(rename = "log4j2-xml")]
    Log4J2Xml,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangLogging {
    pub file: MojangLoggingArtifact,
    pub argument: String,
    #[serde(rename = "type")]
    pub logging_type: MojangLoggingType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MojangArguments {
    pub game: Option<Vec<String>>,
    pub jvm: Option<Vec<serde_json::Value>>,
}

fn default_java_component() -> String {
    "jre-legacy".to_string()
}

fn default_java_version() -> u8 {
    8
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JavaVersion {
    #[serde(default = "default_java_component")]
    pub component: String,
    #[serde(rename = "majorVersion", default = "default_java_version")]
    pub major_version: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<MojangArguments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_index: Option<MojangAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<HashMap<String, MojangArtifactBase>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<MojangLibrary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft_arguments: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::validators::mojang_version_validation",
        default
    )]
    pub minimum_launcher_version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_time: Option<DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherits_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<HashMap<String, MojangLogging>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<JavaVersion>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub version_type: Option<String>,
}
