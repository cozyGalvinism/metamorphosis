use std::collections::HashMap;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use super::{
    misc::GradleSpecifier,
    mojang::{
        JavaVersion, MojangArguments, MojangArtifactBase, MojangAssets, MojangLibrary,
        MojangLogging,
    },
};

#[derive(Clone)]
pub struct FMLLib(String, String, bool);

lazy_static! {
    pub static ref FML_LIB_MAPPING: HashMap<String, Vec<FMLLib>> = {
        let mut m = HashMap::new();

        m.insert(
            "1.3.2".to_string(),
            vec![
                FMLLib(
                    "argo-2.25.jar".to_string(),
                    "bb672829fde76cb163004752b86b0484bd0a7f4b".to_string(),
                    false,
                ),
                FMLLib(
                    "guava-12.0.1.jar".to_string(),
                    "b8e78b9af7bf45900e14c6f958486b6ca682195f".to_string(),
                    false,
                ),
                FMLLib(
                    "asm-all-4.0.jar".to_string(),
                    "98308890597acb64047f7e896638e0d98753ae82".to_string(),
                    false,
                ),
            ],
        );

        let fml14 = vec![
            FMLLib(
                "argo-2.25.jar".to_string(),
                "bb672829fde76cb163004752b86b0484bd0a7f4b".to_string(),
                false,
            ),
            FMLLib(
                "guava-12.0.1.jar".to_string(),
                "b8e78b9af7bf45900e14c6f958486b6ca682195f".to_string(),
                false,
            ),
            FMLLib(
                "asm-all-4.0.jar".to_string(),
                "98308890597acb64047f7e896638e0d98753ae82".to_string(),
                false,
            ),
            FMLLib(
                "bcprov-jdk15on-147.jar".to_string(),
                "b6f5d9926b0afbde9f4dbe3db88c5247be7794bb".to_string(),
                false,
            ),
        ];
        m.insert("1.4".to_string(), fml14.clone());
        m.insert("1.4.1".to_string(), fml14.clone());
        m.insert("1.4.2".to_string(), fml14.clone());
        m.insert("1.4.3".to_string(), fml14.clone());
        m.insert("1.4.4".to_string(), fml14.clone());
        m.insert("1.4.5".to_string(), fml14.clone());
        m.insert("1.4.6".to_string(), fml14.clone());
        m.insert("1.4.7".to_string(), fml14);

        m.insert(
            "1.5".to_string(),
            vec![
                FMLLib(
                    "argo-small-3.2.jar".to_string(),
                    "58912ea2858d168c50781f956fa5b59f0f7c6b51".to_string(),
                    false,
                ),
                FMLLib(
                    "guava-14.0-rc3.jar".to_string(),
                    "931ae21fa8014c3ce686aaa621eae565fefb1a6a".to_string(),
                    false,
                ),
                FMLLib(
                    "asm-all-4.1.jar".to_string(),
                    "054986e962b88d8660ae4566475658469595ef58".to_string(),
                    false,
                ),
                FMLLib(
                    "bcprov-jdk15on-148.jar".to_string(),
                    "960dea7c9181ba0b17e8bab0c06a43f0a5f04e65".to_string(),
                    true,
                ),
                FMLLib(
                    "deobfuscation_data_1.5.zip".to_string(),
                    "5f7c142d53776f16304c0bbe10542014abad6af8".to_string(),
                    false,
                ),
                FMLLib(
                    "scala-library.jar".to_string(),
                    "458d046151ad179c85429ed7420ffb1eaf6ddf85".to_string(),
                    true,
                ),
            ],
        );

        m.insert(
            "1.5.1".to_string(),
            vec![
                FMLLib(
                    "argo-small-3.2.jar".to_string(),
                    "58912ea2858d168c50781f956fa5b59f0f7c6b51".to_string(),
                    false,
                ),
                FMLLib(
                    "guava-14.0-rc3.jar".to_string(),
                    "931ae21fa8014c3ce686aaa621eae565fefb1a6a".to_string(),
                    false,
                ),
                FMLLib(
                    "asm-all-4.1.jar".to_string(),
                    "054986e962b88d8660ae4566475658469595ef58".to_string(),
                    false,
                ),
                FMLLib(
                    "bcprov-jdk15on-148.jar".to_string(),
                    "960dea7c9181ba0b17e8bab0c06a43f0a5f04e65".to_string(),
                    true,
                ),
                FMLLib(
                    "deobfuscation_data_1.5.1.zip".to_string(),
                    "22e221a0d89516c1f721d6cab056a7e37471d0a6".to_string(),
                    false,
                ),
                FMLLib(
                    "scala-library.jar".to_string(),
                    "458d046151ad179c85429ed7420ffb1eaf6ddf85".to_string(),
                    true,
                ),
            ],
        );

        m.insert(
            "1.5.2".to_string(),
            vec![
                FMLLib(
                    "argo-small-3.2.jar".to_string(),
                    "58912ea2858d168c50781f956fa5b59f0f7c6b51".to_string(),
                    false,
                ),
                FMLLib(
                    "guava-14.0-rc3.jar".to_string(),
                    "931ae21fa8014c3ce686aaa621eae565fefb1a6a".to_string(),
                    false,
                ),
                FMLLib(
                    "asm-all-4.1.jar".to_string(),
                    "054986e962b88d8660ae4566475658469595ef58".to_string(),
                    false,
                ),
                FMLLib(
                    "bcprov-jdk15on-148.jar".to_string(),
                    "960dea7c9181ba0b17e8bab0c06a43f0a5f04e65".to_string(),
                    true,
                ),
                FMLLib(
                    "deobfuscation_data_1.5.2.zip".to_string(),
                    "446e55cd986582c70fcf12cb27bc00114c5adfd9".to_string(),
                    false,
                ),
                FMLLib(
                    "scala-library.jar".to_string(),
                    "458d046151ad179c85429ed7420ffb1eaf6ddf85".to_string(),
                    true,
                ),
            ],
        );

        m
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeFile {
    pub classifier: String,
    pub hash: String,
    pub extension: String,
}

impl ForgeFile {
    pub fn file_name(&self, long_version: &str) -> String {
        format!(
            "forge-{}-{}.{}",
            long_version, self.classifier, self.extension
        )
    }

    pub fn url(&self, long_version: &str) -> String {
        format!(
            "https://files.minecraftforge.net/maven/net/minecraftforge/forge/{}/{}",
            long_version,
            self.file_name(long_version)
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeEntry {
    #[serde(rename = "longversion")]
    pub long_version: String,
    #[serde(rename = "mcversion")]
    pub mc_version: String,
    pub version: String,
    pub build: i32,
    pub branch: Option<String>,
    pub latest: Option<bool>,
    pub recommended: Option<bool>,
    pub files: Option<HashMap<String, ForgeFile>>,
}

pub struct ForgeVersion {
    pub build: i32,
    pub raw_version: String,
    pub mc_version: String,
    pub mc_version_sane: String,
    pub branch: Option<String>,
    pub installer_file_name: Option<String>,
    pub installer_url: Option<String>,
    pub universal_file_name: Option<String>,
    pub universal_url: Option<String>,
    pub changelog_url: Option<String>,
    pub long_version: String,
}

impl From<ForgeEntry> for ForgeVersion {
    fn from(entry: ForgeEntry) -> Self {
        let build = entry.build;
        let raw_version = entry.version;
        let mc_version = entry.mc_version;
        let mc_version_sane = mc_version.replacen("_pre", "-pre", 1);
        let branch = entry.branch;
        let mut long_version = format!("{}-{}", mc_version, raw_version);

        if let Some(branch) = &branch {
            long_version = format!("{}-{}", long_version, branch);
        }

        let mut installer_file_name: Option<String> = None;
        let mut installer_url: Option<String> = None;
        let mut universal_file_name: Option<String> = None;
        let mut universal_url: Option<String> = None;
        let mut changelog_url: Option<String> = None;

        if let Some(files) = entry.files {
            for (classifier, file_entry) in files {
                let extension = file_entry.extension.clone();
                let file_name = file_entry.file_name(long_version.as_str());
                let url = file_entry.url(long_version.as_str());

                if classifier == "installer" && extension == "jar" {
                    installer_file_name = Some(file_name);
                    installer_url = Some(url);
                    continue;
                }

                if (classifier == "universal" || classifier == "client") && (extension == "jar" || extension == "zip") {
                    universal_file_name = Some(file_name);
                    universal_url = Some(url);
                    continue;
                }

                if classifier == "changelog" && extension == "txt" {
                    changelog_url = Some(url);
                    continue;
                }
            }
        }

        ForgeVersion {
            build,
            raw_version,
            mc_version,
            mc_version_sane,
            branch,
            installer_file_name,
            installer_url,
            universal_file_name,
            universal_url,
            changelog_url,
            long_version,
        }
    }
}

impl ForgeVersion {
    pub fn name(&self) -> String {
        format!("Forge {}", self.build)
    }

    pub fn uses_installer(&self) -> bool {
        if self.installer_url.is_none() {
            return false;
        }

        if self.mc_version == "1.5.2" {
            return false;
        }

        true
    }

    pub fn file_name(&self) -> Option<String> {
        if self.uses_installer() {
            self.installer_file_name.clone()
        } else {
            self.universal_file_name.clone()
        }
    }

    pub fn url(&self) -> Option<String> {
        if self.uses_installer() {
            self.installer_url.clone()
        } else {
            self.universal_url.clone()
        }
    }

    pub fn is_supported(&self) -> bool {
        if self.url().is_none() {
            return false;
        }

        let version_elements = self.raw_version.split('.').collect::<Vec<&str>>();
        if version_elements.is_empty() {
            return false;
        }
        
        let major_version = version_elements[0];
        if major_version.parse::<i32>().is_err() {
            return false;
        }

        true
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeMCVersionInfo {
    pub latest: Option<String>,
    pub recommended: Option<String>,
    pub versions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DerivedForgeIndex {
    pub mc_versions: Option<HashMap<String, ForgeMCVersionInfo>>,
    pub versions: Option<HashMap<String, ForgeEntry>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForgeInstallerProfileInstallSection {
    pub profile_name: String,
    pub target: String,
    pub path: GradleSpecifier,
    pub version: String,
    pub file_path: String,
    pub welcome: String,
    pub minecraft: String,
    pub logo: String,
    pub mirror_list: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_list: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeLibrary {
    #[serde(flatten)]
    pub library: MojangLibrary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "serverreq", skip_serializing_if = "Option::is_none")]
    pub server_req: Option<bool>,
    #[serde(rename = "clientreq", skip_serializing_if = "Option::is_none")]
    pub client_req: Option<bool>,
    pub checksums: Option<Vec<String>>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForgeVersionFile {
    // flatten doesn't work here, because it can't handle overriding fields
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
    pub libraries: Option<Vec<ForgeLibrary>>,
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
    pub inherits_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<HashMap<String, MojangLogging>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<JavaVersion>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub version_type: Option<String>,
    pub jar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeOptional {
    pub name: Option<String>,
    pub client: Option<bool>,
    pub server: Option<bool>,
    pub default: Option<bool>,
    pub inject: Option<bool>,
    #[serde(rename = "desc")]
    pub description: Option<String>,
    pub url: Option<String>,
    pub artifact: Option<GradleSpecifier>,
    pub maven: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeInstallerProfile {
    pub install: ForgeInstallerProfileInstallSection,
    #[serde(rename = "versionInfo")]
    pub version_info: ForgeVersionFile,
    pub optionals: Option<Vec<ForgeOptional>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeLegacyInfo {
    #[serde(rename = "releaseTime")]
    pub release_time: Option<DateTime<chrono::Utc>>,
    pub size: Option<i32>,
    pub sha256: Option<String>,
    pub sha1: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeLegacyInfoList {
    pub number: Option<HashMap<String, ForgeLegacyInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSpec {
    pub client: Option<String>,
    pub server: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessorSpec {
    pub jar: Option<String>,
    pub classpath: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub outputs: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sides: Option<Vec<String>>,
}

/// A Forge installer profile, which is only ever used in 1.12.2-14.23.5.2851
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeInstallerProfileV1_5 {
    pub _comment: Option<Vec<String>>,
    pub spec: Option<i32>,
    pub profile: Option<String>,
    pub version: Option<String>,
    pub icon: Option<String>,
    pub json: Option<String>,
    pub path: Option<GradleSpecifier>,
    pub logo: Option<String>,
    pub minecraft: Option<String>,
    pub welcome: Option<String>,
    pub data: Option<serde_json::Value>,
    pub processors: Option<Vec<ProcessorSpec>>,
    pub libraries: Option<Vec<MojangLibrary>>,
    #[serde(rename = "mirrorList", skip_serializing_if = "Option::is_none")]
    pub mirror_list: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgeInstallerProfileV2 {
    pub _comment: Option<Vec<String>>,
    pub spec: Option<i32>,
    pub profile: Option<String>,
    pub version: Option<String>,
    pub icon: Option<String>,
    pub json: Option<String>,
    pub path: Option<GradleSpecifier>,
    pub logo: Option<String>,
    pub minecraft: Option<String>,
    pub welcome: Option<String>,
    pub data: Option<HashMap<String, DataSpec>>,
    pub processors: Option<Vec<ProcessorSpec>>,
    pub libraries: Option<Vec<MojangLibrary>>,
    #[serde(rename = "mirrorList", skip_serializing_if = "Option::is_none")]
    pub mirror_list: Option<String>,
    #[serde(rename = "serverJarPath", skip_serializing_if = "Option::is_none")]
    pub server_jar_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstallerInfo {
    #[serde(rename = "sha1hash")]
    pub sha1_hash: Option<String>,
    #[serde(rename = "sha256hash")]
    pub sha256_hash: Option<String>,
    pub size: Option<u64>,
}
