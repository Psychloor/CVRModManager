use std::{collections::HashMap, sync::LazyLock};

pub static AUTHORS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert("<@!170953680718266369>".to_string(), "ImTiara".to_string());
    map.insert("<@!286669951987613706>".to_string(), "Rafa".to_string());
    map.insert("<@!168795588366696450>".to_string(), "Grummus".to_string());
    map.insert(
        "<@!167335587488071682>".to_string(),
        "KortyBoi/Lily".to_string(),
    );
    map.insert("<@!127978642981650432>".to_string(), "tetra".to_string());
    map.insert(
        "<@!155396491853168640>".to_string(),
        "Dawn/arion".to_string(),
    );

    map
});

pub fn get_author_name(discord_id: &str) -> Option<&str> {
    if let Some(name) = AUTHORS.get(discord_id) {
        return Some(name);
    } else {
        None
    }
}
