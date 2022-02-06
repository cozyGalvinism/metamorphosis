use serde::{Deserialize, Serialize};

use super::polymc::PolyMCLibrary;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricInstallerArguments {
    pub client: Option<Vec<String>>,
    pub common: Option<Vec<String>>,
    pub server: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricInstallerLaunchWrapper {
    pub tweakers: FabricInstallerArguments,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricInstallerLibraries {
    pub client: Option<Vec<PolyMCLibrary>>,
    pub common: Option<Vec<PolyMCLibrary>>,
    pub server: Option<Vec<PolyMCLibrary>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricInstallerDataV1 {
    pub version: i32,
    pub libraries: FabricInstallerLibraries,
    #[serde(rename = "mainClass")]
    pub main_class: serde_json::Value,
    pub arguments: Option<FabricInstallerArguments>,
    #[serde(rename = "launchwrapper")]
    pub launch_wrapper: Option<FabricInstallerLaunchWrapper>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricJarInfo {
    #[serde(rename = "releaseTime")]
    pub release_time: Option<chrono::DateTime<chrono::Utc>>,
    pub size: Option<u64>,
    pub sha256: Option<String>,
    pub sha1: Option<String>,
}
