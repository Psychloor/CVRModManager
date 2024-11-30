use crate::api::api_error::ApiError;
use crate::api::mod_info::ModInfo;
use crate::api::mod_version::ModType;
use crate::sha256_hasher;
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

pub(crate) mod api_error;
pub(crate) mod mod_info;
pub(crate) mod mod_version;

const API_DOMAIN_URL: &'static str = "https://api.cvrmg.com";
const API_VERSION: &'static str = "v1";
const USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[cfg(debug_assertions)]
const MODS_JSON: &'static str = include_str!("mods.json");

fn get_mods_api_url() -> String {
    format!("{}/{}/mods/", API_DOMAIN_URL, API_VERSION)
}

fn create_client() -> reqwest::Result<Client> {
    Client::builder()
        .user_agent(USER_AGENT)
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                "User-Agent",
                USER_AGENT.parse().expect("User-Agent not valid header"),
            );
            headers
        })
        .build()
}

pub(crate) async fn fetch_all_mods() -> Result<Vec<ModInfo>, ApiError> {
    if cfg!(debug_assertions) {
        let mods: Vec<ModInfo> = serde_json::from_str(MODS_JSON)?;
        Ok(mods)
    } else {
        let client = create_client()?;
        let response = client.get(get_mods_api_url()).send().await?;
        let mods: Vec<ModInfo> = response.json().await?;
        Ok(mods)
    }
}

fn get_file_name_from_url(response: &reqwest::Response) -> Option<String> {
    let remote_url = response.url().to_string();

    remote_url
        .split('/')
        .filter(|s| !s.is_empty()) // Ignore empty segments
        .last()
        .filter(|&s| s.ends_with(".dll"))
        .map(|s| s.to_string())
}

pub(crate) async fn download_and_verify_mod_with_info<P: Into<PathBuf>>(
    mod_info: &ModInfo,
    loader_path: P,
) -> Result<(), ApiError> {
    let mod_version = &mod_info.versions[0];
    download_and_verify_mod(
        mod_version.download_link.as_str(),
        mod_version.hash.as_str(),
        &mod_version.mod_type,
        loader_path,
    )
    .await
}

pub(crate) async fn download_and_verify_mod<P: Into<PathBuf>>(
    mod_url: &str,
    mod_hash: &str,
    mod_type: &ModType,
    loader_path: P,
) -> Result<(), ApiError> {
    let client = create_client()?;
    let response = client.get(mod_url).send().await?;

    // Use error_for_status to handle non-success statuses
    if let Err(err) = response.error_for_status_ref() {
        return Err(err.into());
    }

    if let Some(file_name) = get_file_name_from_url(&response) {
        let bytes = response.bytes().await?;

        sha256_hasher::compute_sha256_hash(&bytes)
            .await
            .ok_or_else(|| ApiError::InvalidFileHash)
            .and_then(|file_hash| {
                if file_hash.eq(mod_hash) {
                    Ok(())
                } else {
                    Err(ApiError::InvalidFileHash)
                }
            })?;

        let file_path = match mod_type {
            ModType::Mod => loader_path.into().join("Mods"),
            ModType::Plugin => loader_path.into().join("Plugins"),
        };

        let file_path = file_path.join(file_name);
        let mut file = crate::utils::create_file_with_directories(&file_path).await?;

        file.write_all(&bytes).await?;
        file.flush().await?;

        Ok(())
    } else {
        Err(ApiError::InvalidFileName)
    }
}
