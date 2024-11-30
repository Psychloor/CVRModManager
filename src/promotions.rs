pub struct Promotion {
    pub mod_name: String,
    pub description: String,
    pub link: String,
}

impl Promotion {
    pub fn new(mod_name: String, description: String, link: String) -> Self {
        Self {
            mod_name,
            description,
            link,
        }
    }
}

pub struct Promotions {
    pub promotions: Vec<Promotion>,
}

impl Promotions {
    pub fn new() -> Self {
        let mut promotions = Vec::new();

        promotions.push(Promotion::new(
            "Chillout Modding Group".to_string(),
            "Join our Discord!".to_string(),
            "https://discord.gg/dndGPM3bxu".to_string(),
        ));

        Self { promotions }
    }
}
