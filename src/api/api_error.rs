use crate::api::mod_info::ModInfo;
use reqwest::header::HeaderMap;
use reqwest::{Client, Request};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::path::Path;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

pub(crate) mod mod_info;
pub(crate) mod mod_version;

//noinspection SpellCheckingInspection
#[derive(Debug)]
pub(crate) enum ApiError {
    ReqwestError(reqwest::Error),
    IOError(std::io::Error),
    InvalidFileName,
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            ApiError::IOError(err) => write!(f, "IO error: {}", err),
            ApiError::InvalidFileName => write!(f, "Invalid file name"),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			ApiError::ReqwestError(err) => Some(err),
			ApiError::IOError(err) => Some(err),
			_ => None,
		}
	}
}

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
            headers.insert("User-Agent", USER_AGENT.parse().unwrap());
            headers
        })
        .build()
}

pub(crate) async fn get_all_mods() -> Result<Vec<ModInfo>, ApiError> {
    if cfg!(debug_assertions) {
        let mods: Vec<ModInfo> = serde_json::from_str(MODS_JSON).unwrap();
        Ok(mods)
    } else {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());

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

pub(crate) async fn download_mod(mod_url: &str, mod_folder_path: &Path) -> Result<(), ApiError> {
    let client = create_client()?;
    let response = client.get(mod_url).send().await?;

    // Use error_for_status to handle non-success statuses
    if let Err(err) = response.error_for_status_ref() {
        return Err(ApiError::from(err));
    }

    if let Some(file_name) = get_file_name_from_url(&response) {
        let file_path = mod_folder_path.join(file_name);
        let mut file = File::create(file_path).await?;
        let bytes = response.bytes().await?;
        file.write_all(&bytes).await?;
        Ok(())
    } else {
        Err(ApiError::InvalidFileName)
    }
}