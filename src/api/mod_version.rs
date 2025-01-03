use semver::Version;
use serde::de::{self};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use super::ApiError;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Default)]
pub(crate) enum ApprovalStatus {
    #[default]
    AwaitingApproval,
    Approved,
    /// Retired too
    Outdated(Option<String>),
    Broken(Option<String>),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize)]
pub(crate) enum ModType {
    Mod,
    Plugin,
}

impl Default for ModType {
    fn default() -> Self {
        Self::Mod
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModVersion {
    #[serde(deserialize_with = "deserialize_approval_status", default)]
    pub approval_status: ApprovalStatus,
    pub name: String,
    #[serde(deserialize_with = "parse_semver")]
    #[allow(clippy::struct_field_names)]
    pub mod_version: Version,
    pub game_version: String,
    pub loader_version: String,
    #[serde(deserialize_with = "deserialize_mod_type")]
    pub mod_type: ModType,
    #[serde(deserialize_with = "parse_authors")]
    pub authors: Vec<String>,
    pub description: String,
    pub download_link: String,
    pub source_link: String,
    pub embed_color: String,
    pub hash: String,
    #[serde(default)]
    pub requirements: Option<Vec<Option<String>>>,
}

impl ModVersion {
    // Converts color hex to [f32; 3]
    pub(crate) fn color_as_f32(&self) -> Result<[f32; 3], ApiError> {
        let max = f32::from(u8::MAX);
        let hex = self.embed_color.trim_start_matches('#');
        if hex.len() == 6 {
            Ok([
                f32::from(u8::from_str_radix(&hex[0..2], 16)?) / max,
                f32::from(u8::from_str_radix(&hex[2..4], 16)?) / max,
                f32::from(u8::from_str_radix(&hex[4..6], 16)?) / max,
            ])
        } else if hex.len() == 3 {
            Ok([
                f32::from(u8::from_str_radix(&format!("{0}{0}", &hex[0..1]), 16)?) / max,
                f32::from(u8::from_str_radix(&format!("{0}{0}", &hex[1..2]), 16)?) / max,
                f32::from(u8::from_str_radix(&format!("{0}{0}", &hex[2..3]), 16)?) / max,
            ])
        } else {
            Err(ApiError::InvalidColorHexLength)
        }
    }

    // Converts color hex to [u8; 3]
    pub(crate) fn color_as_u8(&self) -> Result<[u8; 3], ApiError> {
        let hex = self.embed_color.trim_start_matches('#');
        if hex.len() == 6 {
            Ok([
                u8::from_str_radix(&hex[0..2], 16)?,
                u8::from_str_radix(&hex[2..4], 16)?,
                u8::from_str_radix(&hex[4..6], 16)?,
            ])
        } else if hex.len() == 3 {
            Ok([
                u8::from_str_radix(&format!("{0}{0}", &hex[0..1]), 16)?,
                u8::from_str_radix(&format!("{0}{0}", &hex[1..2]), 16)?,
                u8::from_str_radix(&format!("{0}{0}", &hex[2..3]), 16)?,
            ])
        } else {
            Err(ApiError::InvalidColorHexLength)
        }
    }

    pub(crate) fn get_authors_joined(&self, separator: &str) -> String {
        self.authors.join(separator)
    }

    pub(crate) fn get_requirements(&self) -> Option<Vec<String>> {
        self.requirements.as_ref().and_then(|requirements| {
            let reqs: Vec<String> = requirements
                .iter()
                .filter_map(|req| req.as_ref())
                .filter_map(|req| {
                    // Some use [name](url) so
                    extract_modname(req)
                })
                .collect();

            if reqs.is_empty() {
                None
            } else {
                Some(reqs)
            }
        })
    }
}

fn extract_modname(req: &str) -> Option<String> {
    if req.contains('[') && req.contains(']') {
        // Handle the [modname] case
        req.trim_start_matches('[')
            .split(']')
            .next()
            .map(|name| name.trim().to_string())
            .filter(|name| !name.is_empty())
    } else {
        // Handle the plain modname case
        Some(req.trim().to_string())
    }
}

fn deserialize_mod_type<'de, D>(deserializer: D) -> Result<ModType, D::Error>
where
    D: Deserializer<'de>,
{
    let mod_type: Option<String> = Option::deserialize(deserializer)?;
    match mod_type {
        None => Ok(ModType::Mod),
        Some(t) => {
            if t.trim().eq_ignore_ascii_case("mod") {
                Ok(ModType::Mod)
            } else if t.trim().eq_ignore_ascii_case("plugin") {
                Ok(ModType::Plugin)
            } else {
                Err(de::Error::custom("Invalid modType value"))
            }
        }
    }
}

fn parse_authors<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let authors: Vec<String> = String::deserialize(deserializer)?
        .split([',', '&'])
        .filter(|s| !s.trim().is_empty())
        .map(std::string::ToString::to_string)
        .collect();

    Ok(authors)
}

fn deserialize_approval_status<'de, D>(deserializer: D) -> Result<ApprovalStatus, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ApprovalStatusHelper {
        #[serde(alias = "approvalStatus")]
        approval_status: i32,
        reason: Option<String>,
    }

    let helper = ApprovalStatusHelper::deserialize(deserializer)?;

    match helper.approval_status {
        0 => Ok(ApprovalStatus::AwaitingApproval),
        1 => Ok(ApprovalStatus::Approved),
        2 => Ok(ApprovalStatus::Broken(helper.reason)),
        3 => Ok(ApprovalStatus::Outdated(helper.reason)),

        status => Err(de::Error::custom(format!(
            "Invalid approvalStatus value: {status}"
        ))),
    }
}

// Custom deserialization function that normalizes and parses semver
fn parse_semver<'de, D>(deserializer: D) -> Result<semver::Version, D::Error>
where
    D: Deserializer<'de>,
{
    let version_str: &str = Deserialize::deserialize(deserializer)?;
    let normalized_version = normalize_version(version_str);
    semver::Version::from_str(&normalized_version).map_err(de::Error::custom)
}

// Function to normalize version strings
fn normalize_version(version: &str) -> String {
    let mut parts = version.split('-'); // Split on pre-release part
    let version_part = parts.next().unwrap();

    let mut version_sections: Vec<&str> = version_part.split('.').collect();

    // Handle versions like "1" or "1.0" and fill missing parts with "0"
    while version_sections.len() < 3 {
        version_sections.push("0");
    }

    // Rejoin the normalized version part
    let normalized_version = version_sections.join(".");

    // If there was a pre-release part, append it back
    if let Some(pre_release) = parts.next() {
        format!("{normalized_version}-{pre_release}")
    } else {
        normalized_version
    }
}
