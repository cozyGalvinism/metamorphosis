/// Validation module for serializing and deserializing Mojang format versions.
pub mod mojang_version_validation {
    use serde::Deserialize;

    use crate::models::mojang::MAX_MOJANG_SUPPORTED_VERSION;

    /// Deserializes the Mojang format version.
    /// 
    /// If the version is not supported, an error is returned.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = Option::<i32>::deserialize(deserializer)?;
        if let Some(v) = v {
            if v > *MAX_MOJANG_SUPPORTED_VERSION {
                return Err(serde::de::Error::custom(format!(
                    "mojang format version {} is not supported, max supported version is {}",
                    v, *MAX_MOJANG_SUPPORTED_VERSION
                )));
            } else {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }

    /// Serializes the Mojang format version.
    /// 
    /// If the version is not supported, an error is returned.
    pub fn serialize<S>(version: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(v) = version {
            if v > &*MAX_MOJANG_SUPPORTED_VERSION {
                return Err(serde::ser::Error::custom(format!(
                    "mojang format version {} is not supported, max supported version is {}",
                    v, *MAX_MOJANG_SUPPORTED_VERSION
                )));
            } else {
                serializer.serialize_i32(*v)
            }
        } else {
            serializer.serialize_none()
        }
    }
}

/// Validation module for serializing and deserializing PolyMC format versions.
pub mod polymc_version_validation {
    use serde::Deserialize;

    use crate::models::polymc::CURRENT_POLYMC_FORMAT_VERSION;

    /// Deserializes the PolyMC format version.
    /// 
    /// If the version is not supported, an error is returned.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = u8::deserialize(deserializer)?;
        if v > *CURRENT_POLYMC_FORMAT_VERSION {
            return Err(serde::de::Error::custom(format!(
                "polymc format version {} is not supported, max supported version is {}",
                v, *CURRENT_POLYMC_FORMAT_VERSION
            )));
        } else {
            Ok(v)
        }
    }

    /// Serializes the PolyMC format version.
    /// 
    /// If the version is not supported, an error is returned.
    pub fn serialize<S>(version: &u8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if version > &*CURRENT_POLYMC_FORMAT_VERSION {
            return Err(serde::ser::Error::custom(format!(
                "polymc format version {} is not supported, max supported version is {}",
                version, *CURRENT_POLYMC_FORMAT_VERSION
            )));
        } else {
            serializer.serialize_u8(*version)
        }
    }
}
