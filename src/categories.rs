use std::{collections::HashMap, sync::LazyLock};

pub static CATEGORY_DESCRIPTION: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "Safety & Security".to_string(),
        "Crash less, block annoyances".to_string(),
    );
    map.insert(
        "Core mods and libraries".to_string(),
        "Other mods might require these".to_string(),
    );
    map.insert(
        "All-in-one mods".to_string(),
        "It does a lot of stuff".to_string(),
    );
    map.insert(
        "Camera mods".to_string(),
        "For all your screenshot or streaming needs".to_string(),
    );
    map.insert(
        "Performance & Fidelity".to_string(),
        "Improve performance or make the game look better".to_string(),
    );
    map.insert(
        "Utilities & Tweaks".to_string(),
        "Small mods that address specific issues".to_string(),
    );
    map.insert(
        "Hardware support".to_string(),
        "For all exotic hardware out there".to_string(),
    );
    map.insert(
        "Dynamic bones".to_string(),
        "Mods that affect jiggly bits".to_string(),
    );
    map.insert(
        "World tweaks".to_string(),
        "Change aspects of the world you're in".to_string(),
    );
    map.insert(
        "Fixes".to_string(),
        "It's not a bug, it's a feature".to_string(),
    );
    map.insert(
        "New features & Overhauls".to_string(),
        "Mods that introduce new features or significantly change existing ones".to_string(),
    );
    map.insert(
        "UI mods".to_string(),
        "Modify the user interface or introduce new functionality to it".to_string(),
    );
    map.insert(
        "Movement".to_string(),
        "Move in new exciting ways".to_string(),
    );
    map.insert(
        "Very Niche Mods".to_string(),
        "Only use these if you're really sure you need them".to_string(),
    );

    map
});

pub fn get_category_description(category: &str) -> Option<&str> {
    if let Some(description) = CATEGORY_DESCRIPTION.get(category) {
        return Some(description);
    } else {
        None
    }
}
