use crate::api::mod_version::ModVersion;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Deserialize)]
pub(crate) struct ModInfo {
    #[serde(alias = "_id")]
    pub(crate) id: usize,
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) aliases: Option<Vec<String>>,
    pub(crate) category: Option<String>,
    pub(crate) versions: Vec<ModVersion>,
}

pub(crate) fn into_hashmap(mods: Vec<ModInfo>) -> HashMap<String, ModInfo> {
    mods.into_iter()
        .map(|mod_info| (mod_info.name.clone(), mod_info))
        .collect()
}