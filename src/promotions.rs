use std::sync::LazyLock;

pub struct Promotion {
    pub mod_name: String,
    pub description: String,
    pub link: String,
}

impl Promotion {
    #[must_use]
    pub fn new(mod_name: String, description: String, link: String) -> Self {
        Self {
            mod_name,
            description,
            link,
        }
    }
}

pub static PROMOTIONS: LazyLock<Promotions> = LazyLock::new(Promotions::new);

pub struct Promotions {
    pub promotions: Vec<Promotion>,
}

impl Promotions {
    fn new() -> Self {
        let promotions = vec![
            Promotion::new(
                "ChilloutVR Modding Group".to_string(),
                "Join our Discord!".to_string(),
                "https://discord.gg/dndGPM3bxu".to_string(),
            )
        ];

        Self { promotions }
    }
}
