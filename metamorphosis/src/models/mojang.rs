use std::{cell::RefCell, collections::HashMap, fmt::Display, ops::DerefMut, str::FromStr};

use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize};

lazy_static! {
    pub static ref MAX_MOJANG_SUPPORTED_VERSION: i32 = 21;
}

// TODO: Change the supported version if it changes!
custom_error! {
    /// Errors that can occur when parsing a Mojang's version index.
    pub MojangError
        InvalidGradleSpecifier { specifier: String } = "Invalid Gradle specifier '{specifier}'",
        UnknownComplicanceLevel { compliance_level: i32, max_supported: i32 } = "Unsupported Mojang compliance level: {compliance_level}. Max supported is: {max_supported}"
}

/// A Gradle specifier.
#[derive(Debug, PartialEq, Clone)]
pub struct GradleSpecifier {
    /// Group of the artifact.
    pub group: String,
    /// Artifact name.
    pub artifact: String,
    /// Version of the artifact.
    pub version: String,
    /// File extension of the artifact.
    pub extension: Option<String>,
    /// Classifier of the artifact.
    pub classifier: Option<String>,
}

impl GradleSpecifier {
    /// Returns the file name of the artifact.
    pub fn filename(&self) -> String {
        if let Some(classifier) = &self.classifier {
            format!(
                "{}-{}-{}.{}",
                self.artifact,
                self.version,
                classifier,
                self.extension.as_ref().unwrap_or(&"".to_string())
            )
        } else {
            format!(
                "{}-{}.{}",
                self.artifact,
                self.version,
                self.extension.as_ref().unwrap_or(&"".to_string())
            )
        }
    }

    /// Returns the base path of the artifact.
    pub fn base(&self) -> String {
        format!(
            "{}/{}/{}",
            self.group.replace(".", "/"),
            self.artifact,
            self.version
        )
    }

    /// Returns the full path of the artifact.
    pub fn path(&self) -> String {
        format!("{}/{}", self.base(), self.filename())
    }

    /// Returns `true` if the specifier is a LWJGL artifact.
    pub fn is_lwjgl(&self) -> bool {
        vec![
            "org.lwjgl",
            "org.lwjgl.lwjgl",
            "net.java.jinput",
            "net.java.jutils",
        ]
        .contains(&self.group.as_str())
    }

    /// Returns `true` if the specifier is a Log4j artifact.
    pub fn is_log4j(&self) -> bool {
        vec!["org.apache.logging.log4j"].contains(&self.group.as_str())
    }
}

impl FromStr for GradleSpecifier {
    type Err = MojangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let at_split = s.split('@').collect::<Vec<&str>>();

        let components = at_split
            .first()
            .ok_or(MojangError::InvalidGradleSpecifier {
                specifier: s.to_string(),
            })?
            .split(':')
            .collect::<Vec<&str>>();

        let group = components
            .get(0)
            .ok_or(MojangError::InvalidGradleSpecifier {
                specifier: s.to_string(),
            })?
            .to_string();
        let artifact = components
            .get(1)
            .ok_or(MojangError::InvalidGradleSpecifier {
                specifier: s.to_string(),
            })?
            .to_string();
        let version = components
            .get(2)
            .ok_or(MojangError::InvalidGradleSpecifier {
                specifier: s.to_string(),
            })?
            .to_string();

        let mut extension = Some("jar".to_string());
        if at_split.len() == 2 {
            extension = Some(at_split[1].to_string());
        }

        let classifier: Option<String>;
        if components.len() == 4 {
            classifier = Some(
                components
                    .get(3)
                    .ok_or(MojangError::InvalidGradleSpecifier {
                        specifier: s.to_string(),
                    })?
                    .to_string(),
            );
        } else {
            classifier = None;
        }

        Ok(GradleSpecifier {
            group,
            artifact,
            version,
            extension,
            classifier,
        })
    }
}

impl Display for GradleSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extension = if let Some(ext) = &self.extension {
            if ext != "jar" {
                format!("@{}", ext)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        if let Some(classifier) = self.classifier.as_ref() {
            write!(
                f,
                "{}:{}:{}:{}{}",
                self.group, self.artifact, self.version, classifier, extension
            )
        } else {
            write!(
                f,
                "{}:{}:{}{}",
                self.group, self.artifact, self.version, extension
            )
        }
    }
}

impl Serialize for GradleSpecifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for GradleSpecifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
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
        with = "crate::validators::mojang_version_validation"
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
