#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::correctness)]
#![warn(clippy::complexity)]
#![warn(clippy::suspicious)]
#![allow(dead_code)]

pub(crate) mod api;
pub mod authors;
pub mod categories;
pub mod config;
pub mod promotions;
pub(crate) mod sha256_hasher;
pub mod utils;

#[tokio::main]
async fn main() {
    let mods = api::fetch_all_mods().await.unwrap();

    for m in mods.iter() {
        let _ = api::download_and_verify_mod_with_info(m, "F:\\New folder").await;
        println!("{} {}", m.name, m.id)
    }

    /*let _ = api::download_and_verify_mod(
        "https://api.cvrmg.com/v1/mods/download/1",
        "6iJpW/dHpcwDFwrDDwBrlKobNHnTVckNlLceHbhwst4=",
        &ModType::Mod,
        Path::new("F:\\New folder"),
    )
    .await
    .ok();*/
}
