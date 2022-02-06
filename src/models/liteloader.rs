use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::polymc::PolyMCLibrary;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteloaderRepo {
    pub stream: String,
    #[serde(rename = "type")]
    pub repo_type: String,
    pub url: String,
    pub classifier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiteloaderArtifact {
    pub tweak_class: String,
    pub libraries: Vec<PolyMCLibrary>,
    pub stream: String,
    pub file: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    pub md5: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_jar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_jar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiteloaderDev {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fg_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiteloaderArtifacts {
    #[serde(rename = "com.mumfrey:liteloader")]
    pub liteloader: HashMap<String, LiteloaderArtifact>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiteloaderSnapshot {
    #[serde(flatten)]
    pub artefact: LiteloaderArtifact,
    pub last_successful_build: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiteloaderSnapshots {
    pub libraries: Vec<PolyMCLibrary>,
    #[serde(rename = "com.mumfrey:liteloader")]
    pub liteloader: HashMap<String, LiteloaderSnapshot>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteloaderEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev: Option<LiteloaderDev>,
    pub repo: LiteloaderRepo,
    #[serde(rename = "artefacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<LiteloaderArtifacts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<LiteloaderSnapshots>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiteloaderMeta {
    pub description: String,
    pub authors: String,
    pub url: String,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub updated_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteloaderIndex {
    pub meta: LiteloaderMeta,
    pub versions: Option<HashMap<String, LiteloaderEntry>>,
}